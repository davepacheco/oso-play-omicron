# Divergences from Omicron's Polar file:
# - we won't deal with AnyActor/UnauthenticatedActor here.  All actors will be
#   authenticated to use any resources here.
# - we don't deal with protecting the "Database" as a resource here

#
# General types and rules
#

actor User {}
actor Team {}
actor Service {}

# For any resource, `actor` can perform action `action` on it if their role(s)
# give them the corresponding permission on that resource.
allow(actor: Actor, action: Action, resource: Resource) if
    has_permission(actor, action.to_perm(), resource);

###
# XXX Will probably make heavy use of "Relations" here:
# https://docs.osohq.com/guides/data_filtering.html#relations
# It's not clear how much of this is implemented for the Rust consumer.  We can
# define a policy that uses relations, plus relation rules.  I think what's
# missing is just the ability to define a relation on the PolarClass, which
# would facilitate data filtering.

# RBAC Resourcess

resource Fleet {
	permissions = [
		# Create and list child resources
		"create_organization",
		"list_child",
	];

	roles = [ "admin" ];
	"create_organization" if "admin";
	"list_child" if "admin";
}

resource Organization {
	permissions = [
		# Operations on this resources
		"delete",
		"modify",

		# Create and list child resources
		"create_project",
		"list_child",
	];

	roles = [
		# Full control over this resource
		"admin",

		# Can create and manage child resources, but cannot modify or
		# delete this resource itself
		"collaborator",

		# Can view the resource and its children
		"viewer",
	];

	# Permission assignments
	"list_child" if "viewer";
	"create_project" if "collaborator";
	"delete" if "admin";
	"modify" if "admin";

	# Role definitions
	"viewer" if "collaborator";
	"collaborator" if "admin";

	# Relationships
	relations = { parent: Fleet };
	"viewer" if "admin" on "parent";
}

resource Project {
	permissions = [
		# Operations on this resources
		"delete",
		"modify",

		# Create and list child resources
		"create_instance",
		"create_disk",
		"create_vpc",
		"list_child",
	];

	roles = [
		# Full control over this resource
		"admin",

		# Can create and manage child resources, but cannot modify or
		# delete this resource itself
		"collaborator",

		# Can view the resource and its children
		"viewer",
	];

	# Permission assignments
	"list_child" if "viewer";
	"create_instance" if "collaborator";
	"create_disk" if "collaborator";
	"create_vpc" if "collaborator";
	"delete" if "admin";
	"modify" if "admin";

	# Role definitions
	"viewer" if "collaborator";
	"collaborator" if "admin";

	# Relationships
	relations = { parent: Organization };
	"viewer" if "viewer" on "parent";
}

resource VmInstance {
	permissions = [ "modify", "delete" ];
	roles = [ "admin" ];

	"modify" if "admin";
	"delete" if "admin";

	# Relationships
	relations = { parent: Organization };
	"admin" if "collaborator" on "parent";
}

has_relation(project: Project, "parent", vminstance: VmInstance)
	if vminstance.project = project;
has_relation(organization: Organization, "parent", project: Project)
	if project.organization = organization;
has_relation(fleet: Fleet, "parent", organization: Organization)
	if organization.fleet = fleet;

# XXX What will this really look like?
has_role(actor: User, role: String, resource: Fleet)
	if actor.has_role_fleet(role, resource);
has_role(actor: User, role: String, resource: Organization)
	if actor.has_role_org(role, resource);
has_role(actor: User, role: String, resource: Project)
	if actor.has_role_project(role, resource);
has_role(actor: User, role: String, resource: VmInstance)
	if actor.has_role_vminstance(role, resource);
