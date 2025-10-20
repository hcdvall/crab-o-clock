use leptos::*;
use leptos::prelude::*;
use chrono::{DateTime, Utc, TimeZone};

#[component]
fn App() -> impl IntoView {
    let target = Utc.with_ymd_and_hms(2025, 10, 31, 13, 0, 0).unwrap();
    
    let (countdown, set_countdown) = signal(calculate_countdown(target));
    
    set_interval(
        move || {
            set_countdown.set(calculate_countdown(target));
        },
        std::time::Duration::from_secs(1),
    );
    
    view! {
        <div class="countdown-container">
            <div class="countdown">
                {move || format!(
                    "{:02}:{:02}:{:02}:{:02}",
                    countdown.get().days,
                    countdown.get().hours,
                    countdown.get().minutes,
                    countdown.get().seconds
                )}
            </div>
        </div>
    }
}

#[derive(Clone, Copy)]
struct Countdown {
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

fn calculate_countdown(target: DateTime<Utc>) -> Countdown {
    let now = Utc::now();
    let diff = target - now;
    
    if diff.num_seconds() <= 0 {
        return Countdown {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        };
    }
    
    let total_seconds = diff.num_seconds();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    Countdown {
        days,
        hours,
        minutes,
        seconds,
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}