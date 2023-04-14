use super::*;

pub mod day;

#[function_component]
pub fn Week() -> Html {
    html! {
        <div class="flex gap-2 mt-4">
            <WeekDay name={WeekDayName::Monday} />
            <WeekDay name={WeekDayName::Tuesday} />
            <WeekDay name={WeekDayName::Wednesday} />
            <WeekDay name={WeekDayName::Thursday} />
            <WeekDay name={WeekDayName::Friday} />
            <WeekDay name={WeekDayName::Saturday} />
            <WeekDay name={WeekDayName::Sunday} />
        </div>
    }
}
