mod video;
mod video_details;
mod videos_list;

use video::Video;
use videos_list::VideosList;
use video_details::VideoDetails;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let selected_video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_video_select = {
        let selected_video: UseStateHandle<Option<Video>> = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details: Option<yew::virtual_dom::VNode> = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    let videos: Vec<Video> = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    html! {
        <>
            <h1>{ "Hello World1" }</h1>
            <VideosList videos={videos} on_click={on_video_select.clone()} />
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
