//! Resource impls used by both backends

use lazy_static::lazy_static;
use oso::Class;
use oso::PolarClass;
use uuid::Uuid;

lazy_static! {
    pub static ref COMMON_RESOURCES: [Class; 8] = [
        User::get_polar_class(),
        Team::get_polar_class(),
        Service::get_polar_class(),
        Action::get_polar_class(),
        Fleet::get_polar_class(),
        Organization::get_polar_class(),
        Project::get_polar_class(),
        VmInstance::get_polar_class(),
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct User {
    pub name: &'static str,
}

impl User {
    pub fn new(name: &'static str) -> User {
        User { name }
    }

    fn has_role_fleet(&self, role_name: &str, _: &Fleet) -> bool {
        // XXX
        self.name == "fran" && role_name == "admin"
    }

    fn has_role_organization(&self, role_name: &str, _: &Organization) -> bool {
        // XXX do we need to apply the recursive policies here?  Seems like you
        // shouldn't.
        (role_name == "admin" && self.name == "omar")
            || (role_name == "collaborator" && self.name == "olivia")
            || (role_name == "viewer" && self.name == "oscar")
    }

    fn has_role_project(&self, role_name: &str, _: &Project) -> bool {
        // XXX do we need to apply the recursive policies here?  Seems like you
        // shouldn't.
        (role_name == "admin" && self.name == "page")
            || (role_name == "collaborator" && self.name == "pedro")
            || (role_name == "viewer" && self.name == "pete")
    }

    fn has_role_vminstance(&self, role_name: &str, _: &VmInstance) -> bool {
        // XXX do we need to apply the recursive policies here?  Seems like you
        // shouldn't.
        role_name == "admin" && self.name == "inigo"
    }
}

impl PolarClass for User {
    fn get_polar_class() -> Class {
        Self::get_polar_class_builder()
            .add_method(
                "has_role_fleet",
                |user: &User, role_name: String, fleet: Fleet| {
                    user.has_role_fleet(&role_name, &fleet)
                },
            )
            .add_method(
                "has_role_org",
                |user: &User, role_name: String, organization: Organization| {
                    user.has_role_organization(&role_name, &organization)
                },
            )
            .add_method(
                "has_role_project",
                |user: &User, role_name: String, project: Project| {
                    user.has_role_project(&role_name, &project)
                },
            )
            .add_method(
                "has_role_vminstance",
                |user: &User, role_name: String, vminstance: VmInstance| {
                    user.has_role_vminstance(&role_name, &vminstance)
                },
            )
            .build()
    }
}

#[derive(Clone, Copy, Debug, PolarClass)]
pub struct Team;
#[derive(Clone, Copy, Debug, PolarClass)]
pub struct Service;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    // Common to most resources
    Modify,
    Delete,

    // Common to container resources
    ListChild,

    // Specific to various container resources

    // Fleet
    CreateOrganization,
    // Organizations
    CreateProject,
    // Projects
    CreateDisk,
    CreateInstance,
    CreateVpc,
}

impl Action {
    // XXX Should take the resource?  How do we do that?
    fn to_perm(&self) -> &'static str {
        match self {
            Action::Modify => "modify",
            Action::Delete => "delete",
            Action::ListChild => "list_child",
            Action::CreateOrganization => "create_organization",
            Action::CreateProject => "create_project",
            Action::CreateDisk => "create_disk",
            Action::CreateInstance => "create_instance",
            Action::CreateVpc => "create_vpc",
        }
    }
}

impl oso::PolarClass for Action {
    fn get_polar_class() -> Class {
        Self::get_polar_class_builder()
            .set_equality_check(|a1, a2| a1 == a2)
            .add_method("to_perm", |a: &Action| a.to_perm())
            .build()
    }
}

#[derive(Clone, Copy, Debug, PolarClass)]
pub struct Fleet;

#[derive(Clone, Copy, Debug)]
pub struct Organization {
    pub id: Uuid,
}

impl PolarClass for Organization {
    fn get_polar_class() -> Class {
        Self::get_polar_class_builder()
            .add_attribute_getter("id", |r| r.id.to_string())
            .add_attribute_getter("fleet", |_| Fleet)
            .build()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Project {
    pub id: Uuid,
    pub organization_id: Uuid,
}

impl PolarClass for Project {
    fn get_polar_class() -> Class {
        Self::get_polar_class_builder()
            .add_attribute_getter("id", |r| r.id.to_string())
            .add_attribute_getter("organization_id", |r| {
                r.organization_id.to_string()
            })
            .build()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VmInstance {
    pub id: Uuid,
    pub project_id: Uuid,
}

impl PolarClass for VmInstance {
    fn get_polar_class() -> Class {
        Self::get_polar_class_builder()
            .add_attribute_getter("id", |r| r.id.to_string())
            .add_attribute_getter("project_id", |r| r.project_id.to_string())
            .add_attribute_getter("project", |r| Project {
                id: r.project_id,
                organization_id: r.project_id,
            }) // XXX
            .build()
    }
}
