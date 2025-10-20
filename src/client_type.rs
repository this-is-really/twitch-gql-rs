/// Represents the type of a Twitch client, defining its basic configuration.
pub struct ClientType {
    pub client_url: String,
    pub client_id: String,
    pub user_agent: String,
}

impl ClientType {
    fn new () -> Self {
        ClientType { client_url: String::new(), client_id: String::new(), user_agent: String::new() }
    }
    fn with_url (mut self , url: &str) -> Self {
        self.client_url = url.to_string();
        self
    }
    fn with_id (mut self, id: &str) -> Self {
        self.client_id = id.to_string();
        self
    }
    fn with_user_agent (mut self, user_agent: &[&str]) -> Self {
        let len = user_agent.len();
        let rand = rand::random_range(0..len);
        self.user_agent = user_agent[rand].to_string();
        self
    }


    /// Ready-made version for web client (browser).
    pub fn web () -> ClientType {
        let user_agent = ["Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"];
        let client_type = ClientType::new().with_url("https://www.twitch.tv").with_id("kimne78kx3ncx6brgo4mv6wki5h1ko").with_user_agent(&user_agent);
        client_type
    }

    /// Ready-made configuration for the mobile version of the web (m.twitch.tv).
    pub fn mobile_web () -> ClientType {
        let user_agents = [
            "Mozilla/5.0 (Linux; Android 16) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; SM-A205U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; SM-A102U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; SM-G960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; SM-N960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; LM-Q720) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; Android 16; LM-X420) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.7204.158 Mobile Safari/537.36",
        ];
        let client_type = ClientType::new().with_url("https://m.twitch.tv").with_id("r8s4dac0uhzifbpu9sjdiwzctle17ff").with_user_agent(&user_agents);
        client_type
    }

    /// Configuration for the Twitch Android app (emulates app headers).
    pub fn android_app () -> ClientType {
        let user_agents = [
            "Dalvik/2.1.0 (Linux; U; Android 16; SM-S911B Build/TP1A.220624.014) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; U; Android 16; SM-S938B Build/BP2A.250605.031) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; Android 16; SM-X716N Build/UP1A.231005.007) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; U; Android 15; SM-G990B Build/AP3A.240905.015.A2) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; U; Android 15; SM-G970F Build/AP3A.241105.008) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; U; Android 15; SM-A566E Build/AP3A.240905.015.A2) tv.twitch.android.app/25.3.0/2503006",
            "Dalvik/2.1.0 (Linux; U; Android 14; SM-X306B Build/UP1A.231005.007) tv.twitch.android.app/25.3.0/2503006",
        ];
        let client_type = ClientType::new().with_url("https://www.twitch.tv").with_id("kd1unb4b3q4t58fwlpcbzcbnm76a8fp").with_user_agent(&user_agents);
        client_type
    }

    /// Configuration for Smart TV / Smart Box Twitch client.
    pub fn smartbox () -> ClientType {
        let user_agent = ["Mozilla/5.0 (Linux; Android 7.1; Smart Box C1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"];
        let client_type = ClientType::new().with_url("https://android.tv.twitch.tv").with_id("ue6666qo983tsx6so1t0vnawi233wa").with_user_agent(&user_agent);
        client_type
    }
}