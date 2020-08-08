mod schema {
    pub mod body {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{character_id};
            pub use super::columns::{species};
            pub use super::columns::{body_type};
            pub use super::columns::{hair_style};
            pub use super::columns::{beard};
            pub use super::columns::{eyes};
            pub use super::columns::{accessory};
            pub use super::columns::{hair_color};
            pub use super::columns::{skin};
            pub use super::columns::{eye_color};
            pub use super::table as body;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            character_id,
            species,
            body_type,
            hair_style,
            beard,
            eyes,
            accessory,
            hair_color,
            skin,
            eye_color,
        ) = (
            character_id,
            species,
            body_type,
            hair_style,
            beard,
            eyes,
            accessory,
            hair_color,
            skin,
            eye_color,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Integer,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
            SmallInt,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("body")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (character_id);
            type AllColumns = (
                character_id,
                species,
                body_type,
                hair_style,
                beard,
                eyes,
                accessory,
                hair_color,
                skin,
                eye_color,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (character_id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    character_id,
                    species,
                    body_type,
                    hair_style,
                    beard,
                    eyes,
                    accessory,
                    hair_color,
                    skin,
                    eye_color,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct character_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for character_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        character_id => {
                            let mut debug_trait_builder = f.debug_tuple("character_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for character_id {
                #[inline]
                fn clone(&self) -> character_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for character_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_character_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for character_id {
                    type QueryId = character_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for character_id {
                #[inline]
                fn default() -> character_id {
                    character_id {}
                }
            }
            impl ::diesel::expression::Expression for character_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for character_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("character_id")
                }
            }
            impl SelectableExpression<table> for character_id {}
            impl<QS> AppearsOnTable<QS> for character_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for character_id where
                character_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for character_id where
                character_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for character_id {}
            impl ::diesel::query_source::Column for character_id {
                type Table = table;
                const NAME: &'static str = "character_id";
            }
            impl<T> ::diesel::EqAll<T> for character_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<character_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct species;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for species {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        species => {
                            let mut debug_trait_builder = f.debug_tuple("species");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for species {
                #[inline]
                fn clone(&self) -> species {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for species {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_species() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for species {
                    type QueryId = species;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for species {
                #[inline]
                fn default() -> species {
                    species {}
                }
            }
            impl ::diesel::expression::Expression for species {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for species
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("species")
                }
            }
            impl SelectableExpression<table> for species {}
            impl<QS> AppearsOnTable<QS> for species where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for species
            where
                species: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for species
            where
                species: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for species where
                species: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for species where
                species: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for species {}
            impl ::diesel::query_source::Column for species {
                type Table = table;
                const NAME: &'static str = "species";
            }
            impl<T> ::diesel::EqAll<T> for species
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<species, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for species where Rhs : :: diesel :: expression :: AsExpression < < < species as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for species where Rhs : :: diesel :: expression :: AsExpression < < < species as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for species where Rhs : :: diesel :: expression :: AsExpression < < < species as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for species where Rhs : :: diesel :: expression :: AsExpression < < < species as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct body_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for body_type {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        body_type => {
                            let mut debug_trait_builder = f.debug_tuple("body_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for body_type {
                #[inline]
                fn clone(&self) -> body_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for body_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_body_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for body_type {
                    type QueryId = body_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for body_type {
                #[inline]
                fn default() -> body_type {
                    body_type {}
                }
            }
            impl ::diesel::expression::Expression for body_type {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for body_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("body_type")
                }
            }
            impl SelectableExpression<table> for body_type {}
            impl<QS> AppearsOnTable<QS> for body_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for body_type
            where
                body_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for body_type
            where
                body_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for body_type where
                body_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for body_type where
                body_type: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for body_type {}
            impl ::diesel::query_source::Column for body_type {
                type Table = table;
                const NAME: &'static str = "body_type";
            }
            impl<T> ::diesel::EqAll<T> for body_type
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<body_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for body_type where Rhs : :: diesel :: expression :: AsExpression < < < body_type as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for body_type where Rhs : :: diesel :: expression :: AsExpression < < < body_type as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for body_type where Rhs : :: diesel :: expression :: AsExpression < < < body_type as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for body_type where Rhs : :: diesel :: expression :: AsExpression < < < body_type as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct hair_style;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for hair_style {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        hair_style => {
                            let mut debug_trait_builder = f.debug_tuple("hair_style");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for hair_style {
                #[inline]
                fn clone(&self) -> hair_style {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for hair_style {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_hair_style() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for hair_style {
                    type QueryId = hair_style;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for hair_style {
                #[inline]
                fn default() -> hair_style {
                    hair_style {}
                }
            }
            impl ::diesel::expression::Expression for hair_style {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for hair_style
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("hair_style")
                }
            }
            impl SelectableExpression<table> for hair_style {}
            impl<QS> AppearsOnTable<QS> for hair_style where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for hair_style
            where
                hair_style: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for hair_style
            where
                hair_style: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for hair_style where
                hair_style: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for hair_style where
                hair_style: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for hair_style {}
            impl ::diesel::query_source::Column for hair_style {
                type Table = table;
                const NAME: &'static str = "hair_style";
            }
            impl<T> ::diesel::EqAll<T> for hair_style
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<hair_style, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for hair_style where Rhs : :: diesel :: expression :: AsExpression < < < hair_style as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for hair_style where Rhs : :: diesel :: expression :: AsExpression < < < hair_style as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for hair_style where Rhs : :: diesel :: expression :: AsExpression < < < hair_style as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for hair_style where Rhs : :: diesel :: expression :: AsExpression < < < hair_style as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct beard;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for beard {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        beard => {
                            let mut debug_trait_builder = f.debug_tuple("beard");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for beard {
                #[inline]
                fn clone(&self) -> beard {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for beard {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_beard() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for beard {
                    type QueryId = beard;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for beard {
                #[inline]
                fn default() -> beard {
                    beard {}
                }
            }
            impl ::diesel::expression::Expression for beard {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for beard
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("beard")
                }
            }
            impl SelectableExpression<table> for beard {}
            impl<QS> AppearsOnTable<QS> for beard where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for beard
            where
                beard: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for beard
            where
                beard: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for beard where
                beard: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for beard where
                beard: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for beard {}
            impl ::diesel::query_source::Column for beard {
                type Table = table;
                const NAME: &'static str = "beard";
            }
            impl<T> ::diesel::EqAll<T> for beard
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<beard, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for beard where Rhs : :: diesel :: expression :: AsExpression < < < beard as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for beard where Rhs : :: diesel :: expression :: AsExpression < < < beard as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for beard where Rhs : :: diesel :: expression :: AsExpression < < < beard as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for beard where Rhs : :: diesel :: expression :: AsExpression < < < beard as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct eyes;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for eyes {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        eyes => {
                            let mut debug_trait_builder = f.debug_tuple("eyes");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for eyes {
                #[inline]
                fn clone(&self) -> eyes {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for eyes {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_eyes() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for eyes {
                    type QueryId = eyes;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for eyes {
                #[inline]
                fn default() -> eyes {
                    eyes {}
                }
            }
            impl ::diesel::expression::Expression for eyes {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for eyes
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("eyes")
                }
            }
            impl SelectableExpression<table> for eyes {}
            impl<QS> AppearsOnTable<QS> for eyes where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for eyes
            where
                eyes: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for eyes
            where
                eyes: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for eyes where
                eyes: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for eyes where
                eyes: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for eyes {}
            impl ::diesel::query_source::Column for eyes {
                type Table = table;
                const NAME: &'static str = "eyes";
            }
            impl<T> ::diesel::EqAll<T> for eyes
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<eyes, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for eyes
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<eyes as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for eyes
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<eyes as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for eyes
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<eyes as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for eyes
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<eyes as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct accessory;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for accessory {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        accessory => {
                            let mut debug_trait_builder = f.debug_tuple("accessory");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for accessory {
                #[inline]
                fn clone(&self) -> accessory {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for accessory {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_accessory() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for accessory {
                    type QueryId = accessory;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for accessory {
                #[inline]
                fn default() -> accessory {
                    accessory {}
                }
            }
            impl ::diesel::expression::Expression for accessory {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for accessory
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("accessory")
                }
            }
            impl SelectableExpression<table> for accessory {}
            impl<QS> AppearsOnTable<QS> for accessory where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for accessory
            where
                accessory: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for accessory
            where
                accessory: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for accessory where
                accessory: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for accessory where
                accessory: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for accessory {}
            impl ::diesel::query_source::Column for accessory {
                type Table = table;
                const NAME: &'static str = "accessory";
            }
            impl<T> ::diesel::EqAll<T> for accessory
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<accessory, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for accessory where Rhs : :: diesel :: expression :: AsExpression < < < accessory as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for accessory where Rhs : :: diesel :: expression :: AsExpression < < < accessory as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for accessory where Rhs : :: diesel :: expression :: AsExpression < < < accessory as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for accessory where Rhs : :: diesel :: expression :: AsExpression < < < accessory as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct hair_color;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for hair_color {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        hair_color => {
                            let mut debug_trait_builder = f.debug_tuple("hair_color");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for hair_color {
                #[inline]
                fn clone(&self) -> hair_color {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for hair_color {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_hair_color() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for hair_color {
                    type QueryId = hair_color;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for hair_color {
                #[inline]
                fn default() -> hair_color {
                    hair_color {}
                }
            }
            impl ::diesel::expression::Expression for hair_color {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for hair_color
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("hair_color")
                }
            }
            impl SelectableExpression<table> for hair_color {}
            impl<QS> AppearsOnTable<QS> for hair_color where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for hair_color
            where
                hair_color: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for hair_color
            where
                hair_color: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for hair_color where
                hair_color: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for hair_color where
                hair_color: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for hair_color {}
            impl ::diesel::query_source::Column for hair_color {
                type Table = table;
                const NAME: &'static str = "hair_color";
            }
            impl<T> ::diesel::EqAll<T> for hair_color
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<hair_color, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for hair_color where Rhs : :: diesel :: expression :: AsExpression < < < hair_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for hair_color where Rhs : :: diesel :: expression :: AsExpression < < < hair_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for hair_color where Rhs : :: diesel :: expression :: AsExpression < < < hair_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for hair_color where Rhs : :: diesel :: expression :: AsExpression < < < hair_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct skin;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for skin {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        skin => {
                            let mut debug_trait_builder = f.debug_tuple("skin");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for skin {
                #[inline]
                fn clone(&self) -> skin {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for skin {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_skin() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for skin {
                    type QueryId = skin;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for skin {
                #[inline]
                fn default() -> skin {
                    skin {}
                }
            }
            impl ::diesel::expression::Expression for skin {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for skin
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("skin")
                }
            }
            impl SelectableExpression<table> for skin {}
            impl<QS> AppearsOnTable<QS> for skin where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for skin
            where
                skin: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for skin
            where
                skin: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for skin where
                skin: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for skin where
                skin: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for skin {}
            impl ::diesel::query_source::Column for skin {
                type Table = table;
                const NAME: &'static str = "skin";
            }
            impl<T> ::diesel::EqAll<T> for skin
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<skin, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for skin
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<skin as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for skin
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<skin as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for skin
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<skin as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for skin
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<skin as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct eye_color;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for eye_color {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        eye_color => {
                            let mut debug_trait_builder = f.debug_tuple("eye_color");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for eye_color {
                #[inline]
                fn clone(&self) -> eye_color {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for eye_color {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_eye_color() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for eye_color {
                    type QueryId = eye_color;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for eye_color {
                #[inline]
                fn default() -> eye_color {
                    eye_color {}
                }
            }
            impl ::diesel::expression::Expression for eye_color {
                type SqlType = SmallInt;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for eye_color
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("eye_color")
                }
            }
            impl SelectableExpression<table> for eye_color {}
            impl<QS> AppearsOnTable<QS> for eye_color where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for eye_color
            where
                eye_color: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for eye_color
            where
                eye_color: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for eye_color where
                eye_color: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for eye_color where
                eye_color: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for eye_color {}
            impl ::diesel::query_source::Column for eye_color {
                type Table = table;
                const NAME: &'static str = "eye_color";
            }
            impl<T> ::diesel::EqAll<T> for eye_color
            where
                T: ::diesel::expression::AsExpression<SmallInt>,
                ::diesel::dsl::Eq<eye_color, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for eye_color where Rhs : :: diesel :: expression :: AsExpression < < < eye_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for eye_color where Rhs : :: diesel :: expression :: AsExpression < < < eye_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for eye_color where Rhs : :: diesel :: expression :: AsExpression < < < eye_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for eye_color where Rhs : :: diesel :: expression :: AsExpression < < < eye_color as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod character {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{id};
            pub use super::columns::{player_uuid};
            pub use super::columns::{alias};
            pub use super::columns::{tool};
            pub use super::table as character;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, player_uuid, alias, tool) = (id, player_uuid, alias, tool);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text, Text, Nullable<Text>);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("character")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, player_uuid, alias, tool);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, player_uuid, alias, tool)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct player_uuid;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for player_uuid {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        player_uuid => {
                            let mut debug_trait_builder = f.debug_tuple("player_uuid");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for player_uuid {
                #[inline]
                fn clone(&self) -> player_uuid {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for player_uuid {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_player_uuid() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for player_uuid {
                    type QueryId = player_uuid;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for player_uuid {
                #[inline]
                fn default() -> player_uuid {
                    player_uuid {}
                }
            }
            impl ::diesel::expression::Expression for player_uuid {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for player_uuid
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("player_uuid")
                }
            }
            impl SelectableExpression<table> for player_uuid {}
            impl<QS> AppearsOnTable<QS> for player_uuid where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for player_uuid
            where
                player_uuid: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for player_uuid
            where
                player_uuid: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for player_uuid where
                player_uuid: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for player_uuid where
                player_uuid: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for player_uuid {}
            impl ::diesel::query_source::Column for player_uuid {
                type Table = table;
                const NAME: &'static str = "player_uuid";
            }
            impl<T> ::diesel::EqAll<T> for player_uuid
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<player_uuid, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct alias;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for alias {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        alias => {
                            let mut debug_trait_builder = f.debug_tuple("alias");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for alias {
                #[inline]
                fn clone(&self) -> alias {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for alias {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_alias() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for alias {
                    type QueryId = alias;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for alias {
                #[inline]
                fn default() -> alias {
                    alias {}
                }
            }
            impl ::diesel::expression::Expression for alias {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for alias
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("alias")
                }
            }
            impl SelectableExpression<table> for alias {}
            impl<QS> AppearsOnTable<QS> for alias where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for alias
            where
                alias: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for alias
            where
                alias: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for alias where
                alias: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for alias where
                alias: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for alias {}
            impl ::diesel::query_source::Column for alias {
                type Table = table;
                const NAME: &'static str = "alias";
            }
            impl<T> ::diesel::EqAll<T> for alias
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<alias, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct tool;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for tool {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        tool => {
                            let mut debug_trait_builder = f.debug_tuple("tool");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for tool {
                #[inline]
                fn clone(&self) -> tool {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for tool {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_tool() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for tool {
                    type QueryId = tool;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for tool {
                #[inline]
                fn default() -> tool {
                    tool {}
                }
            }
            impl ::diesel::expression::Expression for tool {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for tool
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("tool")
                }
            }
            impl SelectableExpression<table> for tool {}
            impl<QS> AppearsOnTable<QS> for tool where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for tool
            where
                tool: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for tool
            where
                tool: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for tool where
                tool: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for tool where
                tool: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for tool {}
            impl ::diesel::query_source::Column for tool {
                type Table = table;
                const NAME: &'static str = "tool";
            }
            impl<T> ::diesel::EqAll<T> for tool
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<tool, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod entity {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{entity_id};
            pub use super::table as entity;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (entity_id,) = (entity_id,);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer,);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("entity")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (entity_id);
            type AllColumns = (entity_id,);
            fn primary_key(&self) -> Self::PrimaryKey {
                (entity_id)
            }
            fn all_columns() -> Self::AllColumns {
                (entity_id,)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct entity_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for entity_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        entity_id => {
                            let mut debug_trait_builder = f.debug_tuple("entity_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for entity_id {
                #[inline]
                fn clone(&self) -> entity_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for entity_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_entity_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for entity_id {
                    type QueryId = entity_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for entity_id {
                #[inline]
                fn default() -> entity_id {
                    entity_id {}
                }
            }
            impl ::diesel::expression::Expression for entity_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for entity_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("entity_id")
                }
            }
            impl SelectableExpression<table> for entity_id {}
            impl<QS> AppearsOnTable<QS> for entity_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for entity_id
            where
                entity_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for entity_id
            where
                entity_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for entity_id where
                entity_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for entity_id where
                entity_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for entity_id {}
            impl ::diesel::query_source::Column for entity_id {
                type Table = table;
                const NAME: &'static str = "entity_id";
            }
            impl<T> ::diesel::EqAll<T> for entity_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<entity_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for entity_id where Rhs : :: diesel :: expression :: AsExpression < < < entity_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for entity_id where Rhs : :: diesel :: expression :: AsExpression < < < entity_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for entity_id where Rhs : :: diesel :: expression :: AsExpression < < < entity_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for entity_id where Rhs : :: diesel :: expression :: AsExpression < < < entity_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod inventory {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{character_id};
            pub use super::columns::{items};
            pub use super::table as inventory;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (character_id, items) = (character_id, items);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("inventory")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (character_id);
            type AllColumns = (character_id, items);
            fn primary_key(&self) -> Self::PrimaryKey {
                (character_id)
            }
            fn all_columns() -> Self::AllColumns {
                (character_id, items)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct character_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for character_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        character_id => {
                            let mut debug_trait_builder = f.debug_tuple("character_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for character_id {
                #[inline]
                fn clone(&self) -> character_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for character_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_character_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for character_id {
                    type QueryId = character_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for character_id {
                #[inline]
                fn default() -> character_id {
                    character_id {}
                }
            }
            impl ::diesel::expression::Expression for character_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for character_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("character_id")
                }
            }
            impl SelectableExpression<table> for character_id {}
            impl<QS> AppearsOnTable<QS> for character_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for character_id where
                character_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for character_id where
                character_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for character_id {}
            impl ::diesel::query_source::Column for character_id {
                type Table = table;
                const NAME: &'static str = "character_id";
            }
            impl<T> ::diesel::EqAll<T> for character_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<character_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct items;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for items {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        items => {
                            let mut debug_trait_builder = f.debug_tuple("items");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for items {
                #[inline]
                fn clone(&self) -> items {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for items {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_items() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for items {
                    type QueryId = items;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for items {
                #[inline]
                fn default() -> items {
                    items {}
                }
            }
            impl ::diesel::expression::Expression for items {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for items
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("items")
                }
            }
            impl SelectableExpression<table> for items {}
            impl<QS> AppearsOnTable<QS> for items where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for items
            where
                items: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for items
            where
                items: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for items where
                items: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for items where
                items: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for items {}
            impl ::diesel::query_source::Column for items {
                type Table = table;
                const NAME: &'static str = "items";
            }
            impl<T> ::diesel::EqAll<T> for items
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<items, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod item {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{item_id};
            pub use super::columns::{parent_container_item_id};
            pub use super::columns::{item_definition_id};
            pub use super::columns::{stack_size};
            pub use super::columns::{position};
            pub use super::table as item;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            item_id,
            parent_container_item_id,
            item_definition_id,
            stack_size,
            position,
        ) = (
            item_id,
            parent_container_item_id,
            item_definition_id,
            stack_size,
            position,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text, Nullable<Integer>, Nullable<Text>);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("item")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (item_id);
            type AllColumns = (
                item_id,
                parent_container_item_id,
                item_definition_id,
                stack_size,
                position,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (item_id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    item_id,
                    parent_container_item_id,
                    item_definition_id,
                    stack_size,
                    position,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct item_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for item_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        item_id => {
                            let mut debug_trait_builder = f.debug_tuple("item_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for item_id {
                #[inline]
                fn clone(&self) -> item_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for item_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_item_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for item_id {
                    type QueryId = item_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for item_id {
                #[inline]
                fn default() -> item_id {
                    item_id {}
                }
            }
            impl ::diesel::expression::Expression for item_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for item_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("item_id")
                }
            }
            impl SelectableExpression<table> for item_id {}
            impl<QS> AppearsOnTable<QS> for item_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for item_id
            where
                item_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for item_id
            where
                item_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for item_id where
                item_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for item_id where
                item_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for item_id {}
            impl ::diesel::query_source::Column for item_id {
                type Table = table;
                const NAME: &'static str = "item_id";
            }
            impl<T> ::diesel::EqAll<T> for item_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<item_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for item_id where Rhs : :: diesel :: expression :: AsExpression < < < item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for item_id where Rhs : :: diesel :: expression :: AsExpression < < < item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for item_id where Rhs : :: diesel :: expression :: AsExpression < < < item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for item_id where Rhs : :: diesel :: expression :: AsExpression < < < item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct parent_container_item_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for parent_container_item_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        parent_container_item_id => {
                            let mut debug_trait_builder = f.debug_tuple("parent_container_item_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for parent_container_item_id {
                #[inline]
                fn clone(&self) -> parent_container_item_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for parent_container_item_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_parent_container_item_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for parent_container_item_id {
                    type QueryId = parent_container_item_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for parent_container_item_id {
                #[inline]
                fn default() -> parent_container_item_id {
                    parent_container_item_id {}
                }
            }
            impl ::diesel::expression::Expression for parent_container_item_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for parent_container_item_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("parent_container_item_id")
                }
            }
            impl SelectableExpression<table> for parent_container_item_id {}
            impl<QS> AppearsOnTable<QS> for parent_container_item_id where
                QS: AppearsInFromClause<table, Count = Once>
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for parent_container_item_id
            where
                parent_container_item_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for parent_container_item_id
            where
                parent_container_item_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for parent_container_item_id where
                parent_container_item_id:
                    SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for parent_container_item_id where
                parent_container_item_id:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for parent_container_item_id {}
            impl ::diesel::query_source::Column for parent_container_item_id {
                type Table = table;
                const NAME: &'static str = "parent_container_item_id";
            }
            impl<T> ::diesel::EqAll<T> for parent_container_item_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<parent_container_item_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for parent_container_item_id where Rhs : :: diesel :: expression :: AsExpression < < < parent_container_item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for parent_container_item_id where Rhs : :: diesel :: expression :: AsExpression < < < parent_container_item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for parent_container_item_id where Rhs : :: diesel :: expression :: AsExpression < < < parent_container_item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for parent_container_item_id where Rhs : :: diesel :: expression :: AsExpression < < < parent_container_item_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct item_definition_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for item_definition_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        item_definition_id => {
                            let mut debug_trait_builder = f.debug_tuple("item_definition_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for item_definition_id {
                #[inline]
                fn clone(&self) -> item_definition_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for item_definition_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_item_definition_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for item_definition_id {
                    type QueryId = item_definition_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for item_definition_id {
                #[inline]
                fn default() -> item_definition_id {
                    item_definition_id {}
                }
            }
            impl ::diesel::expression::Expression for item_definition_id {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for item_definition_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("item_definition_id")
                }
            }
            impl SelectableExpression<table> for item_definition_id {}
            impl<QS> AppearsOnTable<QS> for item_definition_id where QS: AppearsInFromClause<table, Count = Once>
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for item_definition_id
            where
                item_definition_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for item_definition_id
            where
                item_definition_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for item_definition_id where
                item_definition_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for item_definition_id where
                item_definition_id:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for item_definition_id {}
            impl ::diesel::query_source::Column for item_definition_id {
                type Table = table;
                const NAME: &'static str = "item_definition_id";
            }
            impl<T> ::diesel::EqAll<T> for item_definition_id
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<item_definition_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct stack_size;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for stack_size {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        stack_size => {
                            let mut debug_trait_builder = f.debug_tuple("stack_size");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for stack_size {
                #[inline]
                fn clone(&self) -> stack_size {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for stack_size {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_stack_size() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for stack_size {
                    type QueryId = stack_size;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for stack_size {
                #[inline]
                fn default() -> stack_size {
                    stack_size {}
                }
            }
            impl ::diesel::expression::Expression for stack_size {
                type SqlType = Nullable<Integer>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for stack_size
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("stack_size")
                }
            }
            impl SelectableExpression<table> for stack_size {}
            impl<QS> AppearsOnTable<QS> for stack_size where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for stack_size
            where
                stack_size: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for stack_size
            where
                stack_size: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for stack_size where
                stack_size: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for stack_size where
                stack_size: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for stack_size {}
            impl ::diesel::query_source::Column for stack_size {
                type Table = table;
                const NAME: &'static str = "stack_size";
            }
            impl<T> ::diesel::EqAll<T> for stack_size
            where
                T: ::diesel::expression::AsExpression<Nullable<Integer>>,
                ::diesel::dsl::Eq<stack_size, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for stack_size where Rhs : :: diesel :: expression :: AsExpression < < < stack_size as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for stack_size where Rhs : :: diesel :: expression :: AsExpression < < < stack_size as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for stack_size where Rhs : :: diesel :: expression :: AsExpression < < < stack_size as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for stack_size where Rhs : :: diesel :: expression :: AsExpression < < < stack_size as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct position;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for position {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        position => {
                            let mut debug_trait_builder = f.debug_tuple("position");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for position {
                #[inline]
                fn clone(&self) -> position {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for position {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_position() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for position {
                    type QueryId = position;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for position {
                #[inline]
                fn default() -> position {
                    position {}
                }
            }
            impl ::diesel::expression::Expression for position {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for position
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("position")
                }
            }
            impl SelectableExpression<table> for position {}
            impl<QS> AppearsOnTable<QS> for position where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for position
            where
                position: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for position
            where
                position: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for position where
                position: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for position where
                position: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for position {}
            impl ::diesel::query_source::Column for position {
                type Table = table;
                const NAME: &'static str = "position";
            }
            impl<T> ::diesel::EqAll<T> for position
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<position, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod loadout {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{id};
            pub use super::columns::{character_id};
            pub use super::columns::{items};
            pub use super::table as loadout;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, character_id, items) = (id, character_id, items);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("loadout")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, character_id, items);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, character_id, items)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct character_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for character_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        character_id => {
                            let mut debug_trait_builder = f.debug_tuple("character_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for character_id {
                #[inline]
                fn clone(&self) -> character_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for character_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_character_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for character_id {
                    type QueryId = character_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for character_id {
                #[inline]
                fn default() -> character_id {
                    character_id {}
                }
            }
            impl ::diesel::expression::Expression for character_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for character_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("character_id")
                }
            }
            impl SelectableExpression<table> for character_id {}
            impl<QS> AppearsOnTable<QS> for character_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for character_id where
                character_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for character_id where
                character_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for character_id {}
            impl ::diesel::query_source::Column for character_id {
                type Table = table;
                const NAME: &'static str = "character_id";
            }
            impl<T> ::diesel::EqAll<T> for character_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<character_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct items;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for items {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        items => {
                            let mut debug_trait_builder = f.debug_tuple("items");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for items {
                #[inline]
                fn clone(&self) -> items {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for items {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_items() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for items {
                    type QueryId = items;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for items {
                #[inline]
                fn default() -> items {
                    items {}
                }
            }
            impl ::diesel::expression::Expression for items {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for items
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("items")
                }
            }
            impl SelectableExpression<table> for items {}
            impl<QS> AppearsOnTable<QS> for items where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for items
            where
                items: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for items
            where
                items: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for items where
                items: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for items where
                items: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for items {}
            impl ::diesel::query_source::Column for items {
                type Table = table;
                const NAME: &'static str = "items";
            }
            impl<T> ::diesel::EqAll<T> for items
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<items, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod stats {
        #![allow(dead_code)]
        use ::diesel::{QuerySource, Table, JoinTo};
        use ::diesel::associations::HasTable;
        use ::diesel::insertable::Insertable;
        use ::diesel::query_builder::*;
        use ::diesel::query_builder::nodes::Identifier;
        use ::diesel::query_source::{AppearsInFromClause, Once, Never};
        use ::diesel::query_source::joins::{Join, JoinOn};
        use ::diesel::sql_types::*;
        pub use self::columns::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::{character_id};
            pub use super::columns::{level};
            pub use super::columns::{exp};
            pub use super::columns::{endurance};
            pub use super::columns::{fitness};
            pub use super::columns::{willpower};
            pub use super::columns::{skills};
            pub use super::table as stats;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            character_id,
            level,
            exp,
            endurance,
            fitness,
            willpower,
            skills,
        ) = (
            character_id,
            level,
            exp,
            endurance,
            fitness,
            willpower,
            skills,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Integer,
            Integer,
            Integer,
            Integer,
            Integer,
            Integer,
            Nullable<Text>,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("stats")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (character_id);
            type AllColumns = (
                character_id,
                level,
                exp,
                endurance,
                fitness,
                willpower,
                skills,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (character_id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    character_id,
                    level,
                    exp,
                    endurance,
                    fitness,
                    willpower,
                    skills,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel::{Expression, SelectableExpression, AppearsOnTable, QuerySource};
            use ::diesel::backend::Backend;
            use ::diesel::query_builder::{QueryFragment, AstPass, SelectStatement};
            use ::diesel::query_source::joins::{Join, JoinOn, Inner, LeftOuter};
            use ::diesel::query_source::{AppearsInFromClause, Once, Never};
            use ::diesel::result::QueryResult;
            use ::diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct character_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for character_id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        character_id => {
                            let mut debug_trait_builder = f.debug_tuple("character_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for character_id {
                #[inline]
                fn clone(&self) -> character_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for character_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_character_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for character_id {
                    type QueryId = character_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for character_id {
                #[inline]
                fn default() -> character_id {
                    character_id {}
                }
            }
            impl ::diesel::expression::Expression for character_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for character_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("character_id")
                }
            }
            impl SelectableExpression<table> for character_id {}
            impl<QS> AppearsOnTable<QS> for character_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for character_id
            where
                character_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for character_id where
                character_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for character_id where
                character_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for character_id {}
            impl ::diesel::query_source::Column for character_id {
                type Table = table;
                const NAME: &'static str = "character_id";
            }
            impl<T> ::diesel::EqAll<T> for character_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<character_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for character_id where Rhs : :: diesel :: expression :: AsExpression < < < character_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct level;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for level {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        level => {
                            let mut debug_trait_builder = f.debug_tuple("level");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for level {
                #[inline]
                fn clone(&self) -> level {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for level {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_level() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for level {
                    type QueryId = level;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for level {
                #[inline]
                fn default() -> level {
                    level {}
                }
            }
            impl ::diesel::expression::Expression for level {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for level
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("level")
                }
            }
            impl SelectableExpression<table> for level {}
            impl<QS> AppearsOnTable<QS> for level where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for level
            where
                level: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for level
            where
                level: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for level where
                level: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for level where
                level: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for level {}
            impl ::diesel::query_source::Column for level {
                type Table = table;
                const NAME: &'static str = "level";
            }
            impl<T> ::diesel::EqAll<T> for level
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<level, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for level where Rhs : :: diesel :: expression :: AsExpression < < < level as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for level where Rhs : :: diesel :: expression :: AsExpression < < < level as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for level where Rhs : :: diesel :: expression :: AsExpression < < < level as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for level where Rhs : :: diesel :: expression :: AsExpression < < < level as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct exp;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for exp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        exp => {
                            let mut debug_trait_builder = f.debug_tuple("exp");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for exp {
                #[inline]
                fn clone(&self) -> exp {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for exp {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_exp() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for exp {
                    type QueryId = exp;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for exp {
                #[inline]
                fn default() -> exp {
                    exp {}
                }
            }
            impl ::diesel::expression::Expression for exp {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for exp
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("exp")
                }
            }
            impl SelectableExpression<table> for exp {}
            impl<QS> AppearsOnTable<QS> for exp where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for exp
            where
                exp: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for exp
            where
                exp: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for exp where
                exp: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for exp where
                exp: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for exp {}
            impl ::diesel::query_source::Column for exp {
                type Table = table;
                const NAME: &'static str = "exp";
            }
            impl<T> ::diesel::EqAll<T> for exp
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<exp, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for exp
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<exp as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for exp
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<exp as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for exp
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<exp as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for exp
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<exp as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct endurance;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for endurance {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        endurance => {
                            let mut debug_trait_builder = f.debug_tuple("endurance");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for endurance {
                #[inline]
                fn clone(&self) -> endurance {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for endurance {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_endurance() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for endurance {
                    type QueryId = endurance;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for endurance {
                #[inline]
                fn default() -> endurance {
                    endurance {}
                }
            }
            impl ::diesel::expression::Expression for endurance {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for endurance
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("endurance")
                }
            }
            impl SelectableExpression<table> for endurance {}
            impl<QS> AppearsOnTable<QS> for endurance where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for endurance
            where
                endurance: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for endurance
            where
                endurance: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for endurance where
                endurance: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for endurance where
                endurance: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for endurance {}
            impl ::diesel::query_source::Column for endurance {
                type Table = table;
                const NAME: &'static str = "endurance";
            }
            impl<T> ::diesel::EqAll<T> for endurance
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<endurance, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for endurance where Rhs : :: diesel :: expression :: AsExpression < < < endurance as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for endurance where Rhs : :: diesel :: expression :: AsExpression < < < endurance as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for endurance where Rhs : :: diesel :: expression :: AsExpression < < < endurance as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for endurance where Rhs : :: diesel :: expression :: AsExpression < < < endurance as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct fitness;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for fitness {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        fitness => {
                            let mut debug_trait_builder = f.debug_tuple("fitness");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for fitness {
                #[inline]
                fn clone(&self) -> fitness {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for fitness {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_fitness() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for fitness {
                    type QueryId = fitness;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for fitness {
                #[inline]
                fn default() -> fitness {
                    fitness {}
                }
            }
            impl ::diesel::expression::Expression for fitness {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for fitness
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("fitness")
                }
            }
            impl SelectableExpression<table> for fitness {}
            impl<QS> AppearsOnTable<QS> for fitness where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for fitness
            where
                fitness: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for fitness
            where
                fitness: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for fitness where
                fitness: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for fitness where
                fitness: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for fitness {}
            impl ::diesel::query_source::Column for fitness {
                type Table = table;
                const NAME: &'static str = "fitness";
            }
            impl<T> ::diesel::EqAll<T> for fitness
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<fitness, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for fitness where Rhs : :: diesel :: expression :: AsExpression < < < fitness as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for fitness where Rhs : :: diesel :: expression :: AsExpression < < < fitness as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for fitness where Rhs : :: diesel :: expression :: AsExpression < < < fitness as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for fitness where Rhs : :: diesel :: expression :: AsExpression < < < fitness as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct willpower;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for willpower {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        willpower => {
                            let mut debug_trait_builder = f.debug_tuple("willpower");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for willpower {
                #[inline]
                fn clone(&self) -> willpower {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for willpower {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_willpower() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for willpower {
                    type QueryId = willpower;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for willpower {
                #[inline]
                fn default() -> willpower {
                    willpower {}
                }
            }
            impl ::diesel::expression::Expression for willpower {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for willpower
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("willpower")
                }
            }
            impl SelectableExpression<table> for willpower {}
            impl<QS> AppearsOnTable<QS> for willpower where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for willpower
            where
                willpower: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for willpower
            where
                willpower: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for willpower where
                willpower: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for willpower where
                willpower: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for willpower {}
            impl ::diesel::query_source::Column for willpower {
                type Table = table;
                const NAME: &'static str = "willpower";
            }
            impl<T> ::diesel::EqAll<T> for willpower
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<willpower, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for willpower where Rhs : :: diesel :: expression :: AsExpression < < < willpower as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for willpower where Rhs : :: diesel :: expression :: AsExpression < < < willpower as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for willpower where Rhs : :: diesel :: expression :: AsExpression < < < willpower as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for willpower where Rhs : :: diesel :: expression :: AsExpression < < < willpower as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            pub struct skills;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for skills {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        skills => {
                            let mut debug_trait_builder = f.debug_tuple("skills");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for skills {
                #[inline]
                fn clone(&self) -> skills {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for skills {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_skills() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for skills {
                    type QueryId = skills;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for skills {
                #[inline]
                fn default() -> skills {
                    skills {}
                }
            }
            impl ::diesel::expression::Expression for skills {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for skills
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("skills")
                }
            }
            impl SelectableExpression<table> for skills {}
            impl<QS> AppearsOnTable<QS> for skills where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for skills
            where
                skills: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for skills
            where
                skills: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for skills where
                skills: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for skills where
                skills: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for skills {}
            impl ::diesel::query_source::Column for skills {
                type Table = table;
                const NAME: &'static str = "skills";
            }
            impl<T> ::diesel::EqAll<T> for skills
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<skills, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    impl ::diesel::JoinTo<character::table> for body::table {
        type FromClause = character::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<body::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: character::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                body::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<body::table> for character::table {
        type FromClause = body::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<body::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: body::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                body::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<character::table> for inventory::table {
        type FromClause = character::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<inventory::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: character::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                inventory::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<inventory::table> for character::table {
        type FromClause = inventory::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<inventory::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: inventory::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                inventory::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<character::table> for loadout::table {
        type FromClause = character::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<loadout::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: character::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                loadout::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<loadout::table> for character::table {
        type FromClause = loadout::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<loadout::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: loadout::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                loadout::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<character::table> for stats::table {
        type FromClause = character::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<stats::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: character::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                stats::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<stats::table> for character::table {
        type FromClause = stats::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<stats::character_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: stats::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                stats::character_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<character::table> for item::table {
        type FromClause = character::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<item::parent_container_item_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: character::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                item::parent_container_item_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<item::table> for character::table {
        type FromClause = item::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<item::parent_container_item_id>,
            ::diesel::expression::nullable::Nullable<
                <character::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: item::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                item::parent_container_item_id.nullable().eq(
                    <character::table as ::diesel::query_source::Table>::primary_key(
                        &character::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<body::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for body::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<character::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for character::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<entity::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for entity::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<inventory::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for inventory::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<item::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for item::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<loadout::table> for stats::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<stats::table> for loadout::table {
        type Count = ::diesel::query_source::Never;
    }
}
