macro_rules! insert {
    (
        parameters = [
            $($param_ident:ident: $param_ty:ty),*
        ];

        $(setup = {$($setup:stmt;)*};)?

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
            $($($setup)*)?
            
            let row: $row = query_as(concat!("INSERT INTO ", $table, " VALUES ($", $first, $(",$", $num),* , ") RETURNING *"))
                $(.bind($bind))*
                .fetch_one(pool)
                .await?;

            $($recurse)?

            Ok(row)
        }
    };
}

macro_rules! delete {
    ($table:literal) => {
        pub async fn delete(
            pool: &PgPool,
            id: i64
        ) -> Result<Self, Error> {
            query_as(concat!("DELETE FROM ", $table, " WHERE id = $1 RETURNING *"))
                .bind(id)
                .fetch_one(pool)
                .await
        }
    };
}

macro_rules! select {
    ($table:literal) => {
        pub async fn select(
            pool: &PgPool,
            id: i64
        ) -> Result<Self, Error> {
            query_as(concat!("SELECT * FROM ", $table, " WHERE id = $1"))
                .bind(id)
                .fetch_one(pool)
                .await
        }
    };
}

macro_rules! update {
    (
        parameters = [
            $($param_ident:ident: $param_ty:ty),*
        ];

        table = $table:literal;

        names = [$first_name:literal $(, $name:literal)*];

        numbers = [$first_num:literal $(, $num:literal)*];

        binds = [
            $($bind:expr),*
        ];
    ) => {
        pub async fn update(
            pool: &PgPool,
            id: i64,
            $($param_ident: $param_ty),*
        ) -> Result<Self, Error> {
            query_as(concat!("UPDATE ", $table, " SET (", $first_name, $(",", $name),*, ") = ($", $first_num, $(",$", $num),*, ") WHERE id = $1 RETURNING *"))
                .bind(id)
                $(.bind($bind))*
                .fetch_one(pool)
                .await
        }
    };
}

pub mod initialize;
pub mod varve;