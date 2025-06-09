use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct AnalyticsDashboard;

#[contracttype]
pub struct TxMetric {
    pub tx_id: Symbol,
    pub user: Address,
    pub value: i128,
    pub protocol: Symbol,
}

#[contractimpl]
impl AnalyticsDashboard {
    fn metrics<'a>(env: &'a Env) -> Vec<'a, TxMetric> {
        env.storage().instance().get::<Vec<TxMetric>>(Symbol::short("metrics")).unwrap_or(Vec::new(&env))
    }

    pub fn log_tx(env: Env, tx_id: Symbol, value: i128, protocol: Symbol) {
        let user = env.invoker();
        let mut metrics = Self::metrics(&env);
        metrics.push_back(TxMetric { tx_id, user, value, protocol });
        env.storage().instance().set(Symbol::short("metrics"), &metrics);
    }

    pub fn get_metrics(env: Env) -> Vec<TxMetric> {
        Self::metrics(&env)
    }
}
