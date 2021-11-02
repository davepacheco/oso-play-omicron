//! Resource impls used by both backends

use lazy_static::lazy_static;
use oso::Class;
use oso::PolarClass;

lazy_static! {
    pub static ref COMMON_RESOURCES: [Class; 8] = [
        User::get_polar_class(),
        Team::get_polar_class(),
        Service::get_polar_class(),
        Action::get_polar_class(),
        Fleet::get_polar_class(),
        Organization::get_polar_class(),
        Project::get_polar_class(),
        Instance::get_polar_class(),
    ];
}

#[derive(PolarClass)]
struct User;
#[derive(PolarClass)]
struct Team;
#[derive(PolarClass)]
struct Service;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Action {
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

#[derive(PolarClass)]
struct Fleet;

#[derive(PolarClass)]
struct Organization;
#[derive(PolarClass)]
struct Project;
#[derive(PolarClass)]
struct Instance;
