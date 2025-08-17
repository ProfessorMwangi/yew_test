use yew::prelude::*;
use wasm_bindgen::JsValue;

#[function_component(LoginForm)]
fn login_form() -> Html {
    let first_name = use_state(|| String::new());
    let last_name = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let form_submitted = use_state(|| false);
    let form_error = use_state(|| None::<String>);

    let update_state = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                state.set(input.value());
            }
        })
    };

    let onsubmit = {
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let email = email.clone();
        let password = password.clone();
        let form_submitted = form_submitted.clone();
        let form_error = form_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Basic validation
            if first_name.is_empty() || last_name.is_empty() || email.is_empty() || password.is_empty() {
                form_error.set(Some("All fields are required".to_string()));
                return;
            }
            
            if !email.contains('@') {
                form_error.set(Some("Please enter a valid email address".to_string()));
                return;
            }
            
            if password.len() < 6 {
                form_error.set(Some("Password must be at least 6 characters".to_string()));
                return;
            }
            
            // Clear any previous errors
            form_error.set(None);
            
            // In a real app, you would send this data to your server
            // For this example, we'll just log it to the console and set the form as submitted
            let user_data = format!(
                "First Name: {}\nLast Name: {}\nEmail: {}\nPassword: {}",
                *first_name, *last_name, *email, *password
            );
            
            web_sys::console::log_1(&JsValue::from_str(&user_data));
            form_submitted.set(true);
        })
    };

    html! {
        <>
            <style>
                {"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                    font-family: system-ui, -apple-system, BlinkMacSystemFont, 
                        'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 
                        'Open Sans', 'Helvetica Neue', sans-serif;
                }
                
                body {
                    background-color: #4b5563;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    min-height: 100vh;
                    padding: 1rem;
                }
                
                .login-container {
                    width: 100%;
                    max-width: 1000px;
                    margin: auto;
                    display: flex;
                    border-radius: 12px;
                    overflow: hidden;
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
                }
                
                .login-form-section {
                    background-color: #2a2e37;
                    padding: 2rem 3rem;
                    width: 60%;
                    color: white;
                }
                
                .background-section {
                    width: 40%;
                    background-image: url('https://images.unsplash.com/photo-1506905925346-21bda4d32df4?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8');
                    background-size: cover;
                    background-position: center;
                    position: relative;
                }
                
                .background-secti
                on::after {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: linear-gradient(rgba(0,0,0,0.1), rgba(0,0,0,0.3));
                }
                
                .app-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 3rem;
                }
                
                .logo {
                    display: flex;
                    align-items: center;
                }
                
                .logo-icon {
                    width: 36px;
                    height: 36px;
                    background-color: #3b82f6;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-weight: bold;
                    margin-right: 10px;
                }
                
                .nav-links a {
                    color: white;
                    text-decoration: none;
                    margin-left: 1.5rem;
                    opacity: 0.8;
                    transition: opacity 0.2s;
                }
                
                .nav-links a:hover {
                    opacity: 1;
                }
                
                .start-text {
                    font-size: 0.8rem;
                    letter-spacing: 1px;
                    text-transform: uppercase;
                    color: #9ca3af;
                    margin-bottom: 0.5rem;
                }
                
                .title {
                    font-size: 2.25rem;
                    font-weight: bold;
                    margin-bottom: 0.5rem;
                }
                
                .title .dot {
                    color: #3b82f6;
                }
                
                .member-text {
                    color: #9ca3af;
                    margin-bottom: 2rem;
                }
                
                .member-text a {
                    color: #3b82f6;
                    text-decoration: none;
                }
                
                .form-row {
                    display: flex;
                    gap: 1rem;
                    margin-bottom: 1rem;
                }
                
                .form-row .input-container {
                    flex: 1;
                }
                
                .input-container {
                    position: relative;
                    margin-bottom: 1rem;
                }
                
                .input-field {
                    width: 100%;
                    padding: 0.75rem 1rem;
                    padding-right: 2.5rem;
                    background-color: #383c47;
                    border: 1px solid transparent;
                    border-radius: 8px;
                    color: white;
                    font-size: 1rem;
                    transition: border-color 0.2s;
                }
                
                .input-field:focus {
                    outline: none;
                    border-color: #3b82f6;
                }
                
                .input-field::placeholder {
                    color: #9ca3af;
                }
                
                .input-icon {
                    position: absolute;
                    right: 1rem;
                    top: 50%;
                    transform: translateY(-50%);
                    color: #9ca3af;
                    pointer-events: none;
                }
                
                .password-container .input-field {
                    border-color: #3b82f6;
                }
                
                .button-group {
                    display: flex;
                    gap: 1rem;
                    margin-top: 2rem;
                }
                
                .button {
                    padding: 0.75rem 1.5rem;
                    border: none;
                    border-radius: 8px;
                    font-size: 1rem;
                    font-weight: 500;
                    cursor: pointer;
                    transition: background-color 0.2s;
                }
                
                .button.primary {
                    background-color: #3b82f6;
                    color: white;
                }
                
                .button.primary:hover {
                    background-color: #2563eb;
                }
                
                .button.secondary {
                    background-color: #4b5563;
                    color: white;
                }
                
                .button.secondary:hover {
                    background-color: #374151;
                }
                
                .brand-logo {
                    font-size: 1.5rem;
                    font-weight: bold;
                    position: absolute;
                    bottom: 2rem;
                    right: 2rem;
                    color: white;
                    z-index: 10;
                }
                
                .error-message {
                    background-color: rgba(239, 68, 68, 0.2);
                    color: #ef4444;
                    padding: 0.75rem;
                    border-radius: 8px;
                    margin-bottom: 1rem;
                }
                
                .success-message {
                    background-color: rgba(34, 197, 94, 0.2);
                    color: #22c55e;
                    padding: 0.75rem;
                    border-radius: 8px;
                    margin-bottom: 1rem;
                }
                
                @media (max-width: 768px) {
                    .login-container {
                        flex-direction: column;
                    }
                    
                    .login-form-section {
                        width: 100%;
                        padding: 1.5rem;
                    }
                    
                    .background-section {
                        display: none;
                    }
                }
                "}
            </style>
            
            <div class="login-container">
                <div class="login-form-section">
                    <div class="app-header">
                        <div class="logo">
                            <div class="logo-icon">{"A"}</div>
                            <span>{"Anywhere app."}</span>
                        </div>
                        <div class="nav-links">
                            <a href="#">{"Home"}</a>
                            <a href="#">{"Join"}</a>
                        </div>
                    </div>
                    
                    {
                        if *form_submitted {
                            html! {
                                <div class="success-message">
                                    {"Your account has been created successfully!"}
                                </div>
                            }
                        } else {
                            html! {
                                <form onsubmit={onsubmit}>
                                    <div class="form-header">
                                        <p class="start-text">{"START FOR FREE"}</p>
                                        <h1 class="title">{"Create new account"}<span class="dot">{"."}</span></h1>
                                        <p class="member-text">{"Already A Member? "}<a href="#">{"Log In"}</a></p>
                                    </div>
                                    
                                    {
                                        if let Some(error) = &*form_error {
                                            html! {
                                                <div class="error-message">
                                                    {error}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    
                                    <div class="form-row">
                                        <div class="input-container">
                                            <input 
                                                type="text" 
                                                placeholder="First name" 
                                                class="input-field"
                                                value={(*first_name).clone()}
                                                oninput={update_state(first_name.clone())}
                                            />
                                            <span class="input-icon">{"👤"}</span>
                                        </div>
                                        
                                        <div class="input-container">
                                            <input 
                                                type="text" 
                                                placeholder="Last name" 
                                                class="input-field"
                                                value={(*last_name).clone()}
                                                oninput={update_state(last_name.clone())}
                                            />
                                            <span class="input-icon">{"👤"}</span>
                                        </div>
                                    </div>
                                    
                                    <div class="input-container">
                                        <input 
                                            type="email" 
                                            placeholder="Email" 
                                            class="input-field"
                                            value={(*email).clone()}
                                            oninput={update_state(email.clone())}
                                        />
                                        <span class="input-icon">{"✉️"}</span>
                                    </div>
                                    
                                    <div class="input-container password-container">
                                        <input 
                                            type="password" 
                                            placeholder="Password" 
                                            class="input-field"
                                            value={(*password).clone()}
                                            oninput={update_state(password.clone())}
                                        />
                                        <span class="input-icon">{"👁️"}</span>
                                    </div>
                                    
                                    <div class="button-group">
                                        <button type="button" class="button secondary">{"Change method"}</button>
                                        <button type="submit" class="button primary">{"Create account"}</button>
                                    </div>
                                </form>
                            }
                        }
                    }
                </div>
                
                <div class="background-section">
                    <div class="brand-logo">{"W"}</div>
                </div>
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <LoginForm />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}