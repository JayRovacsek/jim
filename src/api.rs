use crate::types::Workout;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_workouts(callback: FetchCallback<Vec<Workout>>) -> FetchTask {
    let req =
        Request::get("https://raw.githubusercontent.com/JayRovacsek/jim/master/data/data.json")
            .body(Nothing)
            .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
