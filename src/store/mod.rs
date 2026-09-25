macro_rules! keys {
    ($($name:ident: $type:ty),*) => {
        /// Returns a copy of the contained primary key.
        pub fn id(&self) -> i64 {
            self.id
        }

        $(
            /// Returns a copy of the contained foreign key.
            pub fn $name(&self) -> $type {
                self.$name
            }
        )*
        
    };
}

macro_rules! queries {
    (
        @insert

        $(foreign = $foreign_ident:ident: $foreign_ty:ty;)?

        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;
        
        $(setup = $resolved:ident: $setup:expr;)?

        binds = [$($bind:expr),*];

        $(recurse = $recurse:expr;)?
    ) =>
    {
        pub async fn insert(
            pool: &PgPool,
            $($foreign_ident: $foreign_ty,)?
            $($param_ident: $param_ty)?
        ) -> Result<Self, Error> {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(concat!("INSERT INTO ", $table, " VALUES (DEFAULT, "));
            let mut binds = builder.separated(", ");
            $(binds.push_bind($foreign_ident);)?

            $(let $resolved = ($setup)(pool).await?;)?
            $(binds.push_bind($bind);)*
            builder.push(") RETURNING *");

            let row: Self = builder
                .build_query_as()
                .fetch_one(pool)
                .await?;
            
            $(($recurse)(pool, &row).await?;)?

            Ok(row)
        }
    };

    (
        @delete
        
        table = $table:literal;
    ) =>
    {
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

    (
        @select

        table = $table:literal;
    ) =>
    {
        pub async fn select<T>(
            pool: &PgPool,
            projection: &str,
            clauses: &str
        ) -> Result<Self, Error> {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT ");
            builder.push(projection);
            builder.push(concat!(" FROM ", $table, " "));
            builder.push(clauses);
            
            builder.build_query_as()
                .fetch_one(pool)
                .await
        }

        pub async fn select_query(
            pool: &PgPool,
            projection: &str,
            clauses: &str
        ) -> Result<PgRow, Error> {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT ");
            builder.push(projection);
            builder.push(concat!(" FROM ", $table, " "));
            builder.push(clauses);
            
            builder.build()
                .fetch_one(pool)
                .await
        }

        pub async fn select_query_scalar<T>(
            pool: &PgPool,
            projection: &str,
            clauses: &str
        ) -> Result<T, Error>
        where
            T: for<'r> Decode<'r, Postgres> + Unpin + Send + Type<Postgres>
        {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT ");
            builder.push(projection);
            builder.push(concat!(" FROM ", $table, " "));
            builder.push(clauses);
            
            builder.build_query_scalar()
                .fetch_one(pool)
                .await
        }
    };

    (
        @update
        
        $(foreign = $foreign_ident:ident: $foreign_ty:ty;)?

        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;

        names = [$($name:literal),*];

        $(setup = $resolved:ident: $setup:expr;)?

        binds = [$($bind:expr),*];
    ) =>
    {
        pub async fn update(
            pool: &PgPool,
            id: i64,
            $($foreign_ident: $foreign_ty,)?
            $($param_ident: $param_ty)?
        ) -> Result<Self, Error> {
            $(let _ = &$param_ident;)?
            
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(concat!("UPDATE ", $table, " SET (" ));
            
            let mut names = builder.separated(", ");
            $(names.push(stringify!($foreign_ident));)?
            $(names.push($name);)*
            
            builder.push(") = (");
            
            let mut binds = builder.separated(", ");
            $(
                binds.push_unseparated("COALESCE (");
                binds.push_bind($foreign_ident);
                binds.push(concat!(stringify!($foreign_ident), ")"));
            )?
            
            $(let $resolved = ($setup)(pool).await?;)?
            $(binds.push_bind($bind);)*
            builder.push(") WHERE id = ");
            builder.push_bind(id);
            builder.push(" RETURNING *");

            builder
                .build_query_as()
                .fetch_one(pool)
                .await
        }
    };
    
    (
        $(foreign = $foreign_ident:ident: $foreign_ty:ty;)?
        
        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;

        names = [$($name:literal),*];

        $(setup = $resolved:ident: $setup:expr;)?

        binds = [$($bind:expr),*];

        $(recurse = $recurse:expr;)?
    ) =>
    {
        queries! {
            @insert
    
            $(foreign = $foreign_ident: $foreign_ty;)?
    
            $(param = $param_ident: $param_ty;)?
            
            table = $table;

            $(setup = $resolved: $setup;)?
    
            binds = [$($bind),*];
    
            $(recurse = $recurse;)?
        }

        queries! {
            @delete
                    
            table = $table;
        }

        queries! {
            @select
                                
            table = $table;
        }

        queries! {
            @update
                    
            $(foreign = $foreign_ident: $foreign_ty;)?
    
            $(param = $param_ident: $param_ty;)?
            
            table = $table;
    
            names = [$($name),*];
            
            $(setup = $resolved: $setup;)?
    
            binds = [$($bind),*];
        }
    };
}

pub mod account;
pub mod initialize;
pub mod organization;
pub mod source;
pub mod varve;