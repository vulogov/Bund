extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use prqlc;
use easy_error::{Error, bail};

fn prql_to_internal_sql(query: String) -> Result<String, Error> {
    let prql_opt = prqlc::Options{
        format: true,
        signature_comment: false,
        color: false,
        display: prqlc::DisplayOptions::Plain,
        target: prqlc::Target::Sql(Some(prqlc::sql::Dialect::DuckDb)),
    };
    match prqlc::prql_to_pl(&query) {
        Ok(pl) => {
            match prqlc::pl_to_rq(pl) {
                Ok(rq) => {
                    match prqlc::rq_to_sql(rq, &prql_opt) {
                        Ok(res) => Ok(res),
                        Err(err) => bail!("PRQL.COMPILE(INTERNAL.SQL) returns: {}", err),
                    }
                }
                Err(err) => bail!("PRQL.COMPILE(INTERNAL.RQ) returns: {}", err),
            }
        }
        Err(err) => bail!("PRQL.COMPILE(INTERNAL.PL) returns: {}", err),
    }

}

#[time_graph::instrument]
pub fn stdlib_internal_prql(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline SQL");
    }
    let query_val = match vm.stack.pull() {
        Some(query_val) => query_val,
        None => bail!("SQL: NO DATA #1"),
    };
    let query_str = match query_val.cast_string() {
        Ok(query_str) => query_str,
        Err(err) => bail!("SQL: casting query returns error: {}", err),
    };
    let query_sql_str = match prql_to_internal_sql(query_str) {
        Ok(query_sql_str) => query_sql_str,
        Err(err) => bail!("INTERNALDB.PRQL returns error: {}", err),
    };
    vm.stack.push(Value::from_string(query_sql_str));
    Ok(vm)
}
