macro_rules! keys {
    ($($name:ident)?) => {
        /// Returns a copy of the contained primary key.
        pub fn id(&self) -> i64 {
            self.id
        }

        $(
            /// Returns a copy of the contained foreign key.
            pub fn $name(&self) -> i64 {
                self.$name
            }
        )?
        
    };
}

macro_rules! queries {
    (
        @insert

        $(foreign = $foreign_ident:ident;)?

        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;

        binds = [$($bind:expr),*];

        $(recurse = $recurse:expr;)?
    ) =>
    {
        pub async fn insert(
            pool: &PgPool,
            $($foreign_ident: i64,)?
            $($param_ident: $param_ty)?
        ) -> Result<Self, Error> {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(concat!("INSERT INTO ", $table, " VALUES (DEFAULT, "));
            let mut binds = builder.separated(", ");
            $(binds.push_bind($foreign_ident);)?
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

    (
        @update
        
        $(foreign = $foreign_ident:ident;)?

        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;

        names = [$($name:literal),*];

        binds = [$($bind:expr),*];
    ) =>
    {
        pub async fn update(
            pool: &PgPool,
            id: i64,
            $($foreign_ident: Option<i64>,)?
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
        $(foreign = $foreign_ident:ident;)?
        
        $(param = $param_ident:ident: $param_ty:ty;)?
        
        table = $table:literal;

        names = [$($name:literal),*];

        binds = [$($bind:expr),*];

        $(recurse = $recurse:expr;)?
    ) =>
    {
        queries! {
            @insert
    
            $(foreign = $foreign_ident;)?
    
            $(param = $param_ident: $param_ty;)?
            
            table = $table;
    
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
                    
            $(foreign = $foreign_ident;)?
    
            $(param = $param_ident: $param_ty;)?
            
            table = $table;
    
            names = [$($name),*];
    
            binds = [$($bind),*];
        }
    };
}

pub mod initialize;
pub mod varve;