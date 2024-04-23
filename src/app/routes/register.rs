use std::io::repeat;

use leptos::{component, create_action, create_server_action, create_signal, ev::SubmitEvent, logging::log, server, view, IntoView, ServerFnError, SignalSet};
use leptos_router::{use_navigate, ActionForm, FromFormData, NavigateOptions, A};

use crate::server::auth::api::SignupAction;

#[component]
pub fn Register() -> impl IntoView {

    let (error_msg, set_error_msg) = create_signal(None::<String>);

    let register_signal = create_server_action::<SignupAction>();

    view! {
        <div class="w-full h-full flex flex-row justify-center align-middle">
            <ActionForm
                class="w-80 flex flex-col bg-slate-400 rounded-md p-6 space-y-4"
                action=register_signal
            >
                <h1 class="text-xl">Register</h1>

                <div class="flex flex-col">
                    <label for="username">Benutzername</label>
                    <input id="username" type="text" name="username" placeholder="Max Mustermann"/>
                </div>
                <div class="flex flex-col">
                    <label for="password">Passwort</label>
                    <input id="password" type="password" name="password"/>
                </div>
                <div class="flex flex-col">
                    <label for="repeat_password">Passwort wiederholen</label>
                    <input id="repeat_password" type="password" name="repeat_password"/>
                </div>
                <input
                    type="submit"
                    class="bg-slate-300 border-slate-500 border-2 rounded-sm hover:bg-slate-200"
                    value="Registrieren"
                />
                <div class="text-red-600">{error_msg}</div>
                <A class="text-blue-700 underline" href="/login.html">
                    Zurück zum Login
                </A>
            </ActionForm>
        </div>
    }
}
