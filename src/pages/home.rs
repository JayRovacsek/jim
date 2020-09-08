// src/pages/home.rs
use crate::api;
use crate::types::Workout;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    workouts: Vec<Workout>,
    get_workouts_error: Option<Error>,
    get_workouts_loaded: bool,
}

pub enum Msg {
    GetWorkouts,
    GetWorkoutsSuccess(Vec<Workout>),
    GetWorkoutsError(Error),
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetWorkouts);
        Self {
            state: State {
                workouts: vec![],
                get_workouts_error: None,
                get_workouts_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetWorkouts => {
                self.state.get_workouts_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Workout>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(workouts) => Msg::GetWorkoutsSuccess(workouts),
                                Err(err) => Msg::GetWorkoutsError(err),
                            }
                        });
                self.task = Some(api::get_workouts(handler));
                true
            }
            Msg::GetWorkoutsSuccess(workouts) => {
                self.state.workouts = workouts;
                self.state.get_workouts_loaded = true;
                true
            }
            Msg::GetWorkoutsError(error) => {
                self.state.get_workouts_error = Some(error);
                self.state.get_workouts_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let workouts: Vec<Html> = self
            .state
            .workouts
            .iter()
            // .filter(|x| x.date == today)
            .map(|workout: &Workout| {
                let sets: Vec<Html> = workout
                    .sets
                    .iter()
                    .map(|x| {
                        html! {
                            <li>{String::from(x.exercise.name.clone())}</li>
                        }
                    })
                    .collect::<Vec<Html>>();
                html! {
                  <div>
                    <div> {&workout.person.name}</div>
                    <ol>
                    {sets}
                    </ol>
                  </div>
                }
            })
            .collect();

        if !self.state.get_workouts_loaded {
            html! {
              <div>{"Loading ..."}</div>
            }
        } else if let Some(_) = self.state.get_workouts_error {
            println!("Didn't work");
            html! {
              <div>
                <span>{"Error loading workouts!"}</span>
              </div>
            }
        } else {
            println!("Worked?");
            html! {
              <div>
                <span>{workouts}</span>
              </div>
            }
        }
    }
}
