#[cfg(test)]
mod tests {
    use crate::connpass;

    #[test]
    fn response_200() {
        let response = connpass::event::new().get().unwrap();

        assert_eq!(response.status, 200);

        match response.body {
            Some(result) => {
                assert_eq!(result.results_start, 1);
                assert_eq!(result.results_returned, 1);
                assert_eq!(result.results_available, 10000);

                for event in result.events {
                    assert_eq!(event.event_id, 100000);
                    assert_eq!(event.title, "sample event");
                    assert_eq!(event.catch, "sample event");
                    assert_eq!(event.description, "<p>sample event</p>");
                    assert_eq!(event.event_url, "https://sample.connpass.com/event/100000/");
                    assert_eq!(event.hash_tag, "");
                    assert_eq!(event.started_at, "2099-01-01T01:00:00+09:00");
                    assert_eq!(event.ended_at, "2099-01-01T02:00:00+09:00");
                    assert_eq!(event.event_type, "participation");
                    assert_eq!(event.owner_id, 100000);
                    assert_eq!(event.owner_nickname, "sample chan");
                    assert_eq!(event.owner_display_name, "sample san");
                    assert_eq!(event.accepted, 0);
                    assert_eq!(event.waiting, 0);
                    assert_eq!(event.updated_at, "2019-12-31T20:00:00+09:00");

                    assert_eq!(event.address.expect("error."), "0 X Y Z");
                    assert_eq!(event.place.expect("error."), "sample");
                    assert_eq!(event.lat.expect("error."), "35.000000000000");
                    assert_eq!(event.lon.expect("error."), "139.000000000000");
                    assert_eq!(event.limit.expect("error."), 20);

                    let series = event.series.expect("error.");
                    assert_eq!(series.id, 1000);
                    assert_eq!(series.url, "https://sample.connpass.com/");
                    assert_eq!(series.title, "sample groupe")
                }
            },
            _ => {
                panic!("error.")
            }
        }
    }
}
