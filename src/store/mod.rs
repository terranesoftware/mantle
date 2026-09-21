macro_rules! insert {
    (
        parameters = [
            $($param_ident:ident: $param_ty:ty),*
        ];

        row = $row:ty;

        table = $table:literal;

        columns = [$first:literal $(, $num:literal)*];

        binds = [
            $($bind:expr),*
        ];

        $(recurse = $recurse:block;)?
    ) => {
        pub async fn insert(
            pool: &PgPool,
            $($param_ident: $param_ty),*
        ) -> Result<Self, Error> {
            let row: $row = query_as(concat!("INSERT INTO ", $table, " VALUES ($", $first, $(",$", $num),* , ") RETURNING *"))
                $(
                    .bind($bind)
                )*
                .fetch_one(pool)
                .await?;

            $($recurse)?

            Ok(row)
        }
    };
}

pub mod initialize;
pub mod varve;