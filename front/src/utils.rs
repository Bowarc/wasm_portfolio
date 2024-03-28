use js_sys::Date;

#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(wasm_bindgen::JsValue),
}

pub fn time_since(date: Date) -> String {
    let s =
        |n: i32, time: &str| -> String { format!("{n} {time}{}", if n > 1 { "s" } else { "" }) };

    let seconds = (Date::new_0().get_time() - date.get_time() / 1000.) as i32;

    let mut interval = seconds / 31536000;
    if interval > 1 {
        return s(interval, "year");
    }
    interval = seconds / 2592000;
    if interval > 1 {
        return s(interval, "month");
    }
    interval = seconds / 86400;
    if interval > 1 {
        return s(interval, "day");
    }
    interval = seconds / 3600;
    if interval > 1 {
        return s(interval, "hour");
    }
    interval = seconds / 60;
    if interval > 1 {
        return s(interval, "minute");
    }

    return s(interval, "second");
}
