use crate::store::varve::profile::{authorization::documents::issuers::IssuerRow, location::Location};
use framboid::account::profile::authorization::document::issuer::Issuer;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

impl IssuerRow {
    queries! {
        foreign = document;

        param = issuer: &Issuer;

        table = "issuers";

        names = ["name", "jurisdiction"];

        binds = [issuer.name(), Location::from(issuer.jurisdiction())];
    }
}