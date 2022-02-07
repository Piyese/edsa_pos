use edsa_pos::{pipeline::{custom_date::Date, inventory::Production}, PathOption, fetch_logs};


// production report
pub fn daily_production(from: Date, to: Date) ->Vec<Production> {
    let list = fetch_logs::<Production>(PathOption::Production).unwrap();
    let list_filter = list.into_iter().filter(|f| f.date > from && f.date < to );
    let final_list: Vec<Production> = Vec::from_iter(list_filter);
    final_list
}