use leptos::{component, create_action, create_effect, create_local_resource, create_local_resource_with_initial_value, create_resource, create_server_action, create_signal, logging::log, view, IntoView, Signal};
use leptos_icons::Icon;
use leptos::Suspense;

use crate::server::auth::api::{current_user, CurrentUserAction};

#[component]
pub fn Content() -> impl IntoView {
    // TODO check authentication. Maybe use a specific router for autheticated routes?
    // let username_action = create_server_action::<CurrentUserAction>();

    // let current_user = username_action.value();
        // username_action.dispatch(CurrentUserAction {  });
    let current_user = create_local_resource(|| (), |()| async {
        current_user().await.unwrap_or("Error".to_owned())
    });

    // create_effect(move |_| {
    // });

    let logout = create_action(|_: &()| async {
        ()
    });


    view! {
        <div class="flex flex-col w-[100vw]">
            <div class="w-full h-16 flex flex-row item-center justify-between">
                <Icon icon=icondata::FaMountainSunSolid class="text-4xl" width="2em"/>
                <h1 class="text-4xl">Fancy Name für die Web-Anwendung</h1>
                <div class="flex flex-col justify-items-start">
                    <Suspense fallback = || "Profil lädt..">
                        eingeloggt als {current_user}

                        <button class="text-blue-700 underline cursor-pointer" on:click=|_| {
                            log!("TODO logout");
                        }>
                        Logout
                        </button>
                    </Suspense>
                </div>
            </div>
            <div class="flex flex-row w-full">
                <div class="basis-3/5 bg-green-400 w-full h-64">
                    <h2>Übersicht über Themen zum Web Engineering</h2>
                    "content"
                </div>
                <div class="basis-2/5 flex flex-row justify-center align-middle bg-red-400 w-full">
                    Placeholder
                </div>
            </div>
        </div>
    }
}
