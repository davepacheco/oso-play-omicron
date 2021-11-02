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
# XXX It looks like relations are not implemented in the Rust crate either.

# RBAC Resourcess

resource Fleet {
	permissions = [
		# Create and list child resources
		"create_organization",
		"list_child",
	];

	roles = [ "admin" ];
	"create_organization" if "admin";
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

	# Role relationships
	"viewer" if "collaborator";
	"collaborator" if "admin";


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

	# Role relationships
	"viewer" if "collaborator";
	"collaborator" if "admin";
}

resource Instance {
	permissions = [ "modify", "delete" ];
	roles = [ "admin" ];

	"modify" if "admin";
	"delete" if "admin";
}
