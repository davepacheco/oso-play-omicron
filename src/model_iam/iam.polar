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
	permissions = [ "admin" ];
	roles = [ "admin" ];
	"create_organization" if "admin";
}

resource Organization {
	permissions = [ "create_project"  ];
	roles = [ "admin" ];

	# Administrator permissions
	"create_project" if "admin";
}
