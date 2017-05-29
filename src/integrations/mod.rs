mod transmission;
use config::BotConfig;
use self::transmission::Transmission;
use handle::BotIntegration;

pub enum IntegrationsEnum {
    Transmission,
}

pub struct IntegrationInstances {
    pub transmission: Transmission,
}

impl IntegrationInstances {
    pub fn new(config: &BotConfig) -> IntegrationInstances {
        IntegrationInstances {
            transmission: Transmission::new(config)
        }
    }
}
