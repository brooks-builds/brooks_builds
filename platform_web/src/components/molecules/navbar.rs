use load_dotenv::load_dotenv;
use yew::prelude::*;

load_dotenv!();

#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container-fluid">
                <div>
                    <img class="navbar-brand" src="/static/brooks.png" alt="Brooks' logo" data-test="nav-logo" />
                    <span class="navbar-text" data-test="nav-title">{"Brooks Builds"}</span>
                </div>
                <div>
                    <a href="#" data-test="auth-sign-up">{"Sign Up"}</a>
                </div>
            </div>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_create_login_url() {
    //     let domain = env!("AUTH0_DOMAIN");
    //     let client_id = env!("AUTH0_CLIENT_ID");
    //     let connection = env!("AUTH0_CONNECTION");
    //     let redirect_uri = env!("AUTH0_REDIRECT_URI");
    //     let state = create_state();

    //     let expected_result = format!(
    //         r#"https://{domain}/authorize?
    //         response_type=token&
    //         client_id={client_id}&
    //         connection={connection}&
    //         redirect_uri={redirect_uri}&
    //         scope=openid%20profile%20email&
    //         state={state}"#
    //     );
    // }

    #[test]
    fn test_create_state() {
        let 
    }
}
