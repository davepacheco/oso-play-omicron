//! This repo gives us a place to play with various ways to model Omicron's
//! resources and policies using Oso.  There are two basic approaches that seem
//! worth fleshing out:
//!
//! - "ReBAC" for short: a relationship-based access control model that's closer
//!   to what people use with Zanzibar.  This looks similar to GitHub or Google
//!   Workspace's model.
//! - "IAM" for short: a fairly direct translation of RFD 43, which resembles
//!   GCP and AWS IAM.
//!
//! The "resources" module includes common resources.  The "model_*" modules
//! implement the corresponding approach.  Ideally, these would both produce the
//! same result: an "Oso" instance that we could use to run authorization
//! checks.  Ideally, we could run the same battery of checks from "main" using
//! both approaches and show that they both have the properties that we want.
//!
//! It's helpful to have a specific example in mind.  So let's start with this:
//! we have an Instance that we want to be made available to:
//!
//! - administrators of the whole rack: read only
//! - admins of the organization: read only
//! - admins of the project: read only
//! - owner of the instance: read+write+admin
//! - a specific user: admin
//! - a specific team: read + write
//! - a service account: read only
//!
//! It may make more sense to start simpler for now.

pub mod model_rebac;
pub mod resources;
