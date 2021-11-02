use oso::Oso;
use oso_play_omicron::resources;
use oso_play_omicron::resources::Action;
use std::fmt::Debug;

fn main() {
    let rebac_oso = oso_play_omicron::model_rebac::make_oso();

    run_checks(&rebac_oso.expect("failed to set up rebac Oso"));
}

fn run_checks(oso: &oso::Oso) {
    // Make up some example data:
    // - a Fleet with admin "Fran"
    // - an Organization with admin "Omar", collaborator "Olivia", viewer
    //   "Oscar"
    // - a Project with admin "Page", collaborator "Pedro", viewer "Pete"
    // - an Instance with admin "Inigo"
    let the_fleet = resources::Fleet;
    let organization_id =
        "00000000-0000-0000-0000-000000000000".parse().unwrap();
    let project_id = "11111111-1111-1111-1111-111111111111".parse().unwrap();
    let instance_id = "22222222-2222-2222-2222-222222222222".parse().unwrap();
    let the_organization = resources::Organization { id: organization_id };
    let the_project = resources::Project { id: project_id, organization_id };
    let the_instance = resources::Instance { id: instance_id, project_id };

    let user_fran = resources::User::new("fran");
    let user_omar = resources::User::new("omar");
    let user_olivia = resources::User::new("olivia");
    let user_oscar = resources::User::new("oscar");
    let user_page = resources::User::new("page");
    let user_pedro = resources::User::new("pedro");
    let user_pete = resources::User::new("pete");
    let user_inigo = resources::User::new("inigo");

    // Check all fleet-level permissions
    check_all(
        oso,
        &[user_fran],
        the_fleet,
        &[Action::CreateOrganization, Action::ListChild],
    );
    check_none(
        oso,
        &[
            user_omar,
            user_olivia,
            user_oscar,
            user_page,
            user_pedro,
            user_pete,
            user_inigo,
        ],
        the_fleet,
        &[Action::CreateOrganization, Action::ListChild],
    );

    // Check all organization-level permissions
    check_all(
        oso,
        &[user_omar],
        the_organization,
        &[Action::Delete, Action::Modify],
    );
    check_none(
        oso,
        &[
            user_fran,
            user_olivia,
            user_oscar,
            user_page,
            user_pedro,
            user_pete,
            user_inigo,
        ],
        the_organization,
        &[Action::Delete, Action::Modify],
    );

    check_all(
        oso,
        &[user_omar, user_olivia],
        the_organization,
        &[Action::CreateProject],
    );
    check_none(
        oso,
        &[user_fran, user_oscar, user_page, user_pedro, user_pete, user_inigo],
        the_organization,
        &[Action::CreateProject],
    );

    check_all(
        oso,
        &[user_omar, user_olivia, user_oscar, user_fran],
        the_organization,
        &[Action::ListChild],
    );
    check_none(
        oso,
        &[user_page, user_pedro, user_pete, user_inigo],
        the_organization,
        &[Action::ListChild],
    );
}

fn check_all<T: oso::PolarClass + Clone + Debug + Send + Sync>(
    oso: &Oso,
    users: &[resources::User],
    resource: T,
    actions: &[resources::Action],
) {
    for u in users {
        for a in actions {
            check(true, oso, u, *a, resource.clone());
        }
    }
}

fn check_none<T: oso::PolarClass + Clone + Debug + Send + Sync>(
    oso: &Oso,
    users: &[resources::User],
    resource: T,
    actions: &[resources::Action],
) {
    for u in users {
        for a in actions {
            check(false, oso, u, *a, resource.clone());
        }
    }
}

fn check<T: oso::PolarClass + Debug + Send + Sync>(
    expected_result: bool,
    oso: &Oso,
    user: &resources::User,
    action: resources::Action,
    resource: T,
) {
    eprint!(
        "check: {:?} {:?} {:?} (expected: {}, ",
        user, action, resource, expected_result
    );
    let result = oso
        .is_allowed(user.clone(), action, resource)
        .expect("authz check failed");
    eprintln!("actual: {})", result);
    assert_eq!(expected_result, result);
}
