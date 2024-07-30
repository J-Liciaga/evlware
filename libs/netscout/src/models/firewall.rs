use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallProfile {
    pub tcp_syn_timing: TcpSynTiming,
    pub icmp_response: IcmpResponse,
    pub tcp_flag_behavior: TcpFlagBehavior,
    pub port_knocking: bool,
    pub application_layer: AppLayerFirewall,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TcpSynTiming {
    Normal,
    Delayed,
    Inconsistent,
    Blocked,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IcmpResponse {
    Allowed,
    Blocked,
    RateLimited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TcpFlagBehavior {
    Standard,
    Filtered,
    Unconventional,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppLayerFirewall {
    None,
    Basic,
    Advanced,
}

#[derive(Debug)]
pub enum FirewallStrength {
    Weak,
    Moderate,
    Strong,
}

impl FirewallProfile {
    pub fn is_firewall_detected(
        &self,
    ) -> bool {
        matches!(
            self.tcp_syn_timing,
            TcpSynTiming::Delayed | TcpSynTiming::Inconsistent | TcpSynTiming::Blocked
        ) ||
        matches!(
            self.icmp_response,
            IcmpResponse::Blocked | IcmpResponse::RateLimited
        ) ||
        matches!(
            self.tcp_flag_behavior,
            TcpFlagBehavior::Filtered | TcpFlagBehavior::Unconventional
        ) ||
        self.port_knocking ||
        matches!(
            self.application_layer,
            AppLayerFirewall::Basic | AppLayerFirewall::Advanced
        )
    }

    pub fn get_firewall_strength(
        &self,
    ) -> FirewallStrength {
        let score = self.calculate_firewall_score();

        match score {
            0..=2 => FirewallStrength::Weak,
            3..=5 => FirewallStrength::Moderate,
            _ => FirewallStrength::Strong,
        }
    }

    fn calculate_firewall_score(
        &self,
    ) -> u8 {
        let mut score = 0;

        score += match self.tcp_syn_timing {
            TcpSynTiming::Normal => 0,
            TcpSynTiming::Delayed => 1,
            TcpSynTiming::Inconsistent => 2,
            TcpSynTiming::Blocked => 3,
        };

        score += match self.icmp_response {
            IcmpResponse::Allowed => 0,
            IcmpResponse::RateLimited => 1,
            IcmpResponse::Blocked => 2,
        };

        score += match self.tcp_flag_behavior {
            TcpFlagBehavior::Standard => 0,
            TcpFlagBehavior::Filtered => 1,
            TcpFlagBehavior::Unconventional => 2,
        };

        if self.port_knocking {
            score += 2;
        }

        score += match self.application_layer {
            AppLayerFirewall::None => 0,
            AppLayerFirewall::Basic => 1,
            AppLayerFirewall::Advanced => 2,
        };

        score
    }
}

impl Default for FirewallProfile {
    fn default() -> Self {
        Self {
            tcp_syn_timing: TcpSynTiming::Normal,
            icmp_response: IcmpResponse::Allowed,
            tcp_flag_behavior: TcpFlagBehavior::Standard,
            port_knocking: false,
            application_layer: AppLayerFirewall::None,
        }
    }
}
