use gpui::{
    div, px, rgb, text_field, AnyElement, AppContext, Context, EventEmitter, InteractiveElement,
    IntoElement, ParentElement, Pixels, Render, SharedString, Styled, View, Window,
};

// Evento para notificar que uma query foi executada
#[derive(Debug, Clone, PartialEq)]
pub struct QueryExecuted(pub SharedString);

pub struct QueryPanel {
    query_text: SharedString,
    query_results: SharedString,
    is_loading: bool,
}

impl EventEmitter for QueryPanel {
    type Event = QueryExecuted;
}

impl QueryPanel {
    pub fn new() -> Self {
        Self {
            query_text: SharedString::from("SELECT * FROM users;"),
            query_results: SharedString::empty(),
            is_loading: false,
        }
    }

    pub fn set_query_text(&mut self, text: SharedString, cx: &mut Context<Self>) {
        self.query_text = text;
        cx.notify();
    }

    pub fn execute_query(&mut self, cx: &mut Context<Self>) {
        self.is_loading = true;
        self.query_results = SharedString::from("Executing query...");
        cx.emit(QueryExecuted(this.query_text.clone()));
        cx.notify();

        // Simulate an asynchronous query execution
        cx.spawn(|this, mut cx| async move {
            cx.background_executor().spawn(async move {
                // Simulate network delay
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }).await;

            this.update(|this, cx| {
                // Dummy results for now
                if this.query_text.to_lowercase().starts_with("select * from users") {
                    this.query_results = SharedString::from("id | name | email\n---|---|---\n1  | Alice | alice@example.com\n2  | Bob | bob@example.com");
                } else if this.query_text.to_lowercase().starts_with("select * from orders") {
                    this.query_results = SharedString::from("id | user_id | total\n---|---|---\n101| 1 | 15.99\n102| 2 | 29.50");
                } else {
                    this.query_results = SharedString::from(format!("Query executed:\n{}", this.query_text));
                }
                this.is_loading = false;
                cx.notify();
            }).unwrap();
        }).detach();
    }
}

impl Render for QueryPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .child(
                div() // Query Editor Area
                    .flex_1()
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        text_field("query-editor", self.query_text.clone(), |this, text, cx| {
                            this.set_query_text(text, cx);
                        })
                        .flex_1()
                        .placeholder("Write your SQL query here...")
                        .font_weight(FontWeight::MONO)
                        .text_color(rgb(0xd4d4d4))
                        .bg(rgb(0x252526))
                        .border_1()
                        .border_color(rgb(0x3c3c3c))
                        .rounded_md()
                        .p_2(),
                    )
                    .child(
                        div() // Execute Button
                            .h(px(36.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(rgb(0x007acc))
                            .text_color(rgb(0xffffff))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|this| this.bg(rgb(0x005f99)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.execute_query(cx);
                            }))
                            .when(self.is_loading, |this| this.child("⏳ Executing..."))
                            .when(!self.is_loading, |this| this.child("▶ Execute Query")),
                    ),
            )
            .child(
                div() // Query Results Area
                    .h(px(200.0)) // Fixed height for results
                    .p_3()
                    .border_t_1()
                    .border_color(rgb(0x2d2d2d))
                    .bg(rgb(0x1e1e1e))
                    .overflow_y_scroll()
                    .child(
                        div()
                            .font_weight(FontWeight::MONO)
                            .text_color(rgb(0x4ec9b0))
                            .child(self.query_results.clone()),
                    ),
            )
    }
}
