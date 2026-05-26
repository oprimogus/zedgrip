use gpui::{
    div, prelude::FluentBuilder, px, rgb, AnyElement, App, AppContext, Context, Entity,
    EventEmitter, FontWeight, InteractiveElement, IntoElement, ParentElement, Pixels, Render,
    SharedString, StatefulInteractiveElement, Styled, View, Window,
};
use std::collections::HashMap;

// --- New: AnyPanel Trait ---
// A trait to generalize over different panel types.
// This allows storing different panel Views in a HashMap.
pub trait AnyPanel: Render + 'static {}
impl AnyPanel for ConnectionsPanel {}
impl AnyPanel for SchemaPanel {}
impl AnyPanel for QueryHistoryPanel {}
impl AnyPanel for SnippetsPanel {}

// Enum para identificar os diferentes tipos de painéis
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PanelType {
    Connections,
    Schema,
    QueryHistory,
    Snippets,
}

// Evento para notificar sobre a mudança de largura da sidebar
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarWidthChanged(pub Pixels);

// Estrutura principal da Sidebar
pub struct Sidebar {
    active_panel: PanelType,
    width: Pixels,
    is_collapsed: bool,
    // Use a HashMap to store panels dynamically
    panels: HashMap<PanelType, View<dyn AnyPanel>>,
}

// Implement EventEmitter para que a Sidebar possa emitir eventos
impl EventEmitter for Sidebar {
    type Event = SidebarWidthChanged;
}

impl Sidebar {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let mut panels: HashMap<PanelType, View<dyn AnyPanel>> = HashMap::new();
        panels.insert(
            PanelType::Connections,
            cx.new_view(|_| ConnectionsPanel::new()).into(),
        );
        panels.insert(
            PanelType::Schema,
            cx.new_view(|_| SchemaPanel::new()).into(),
        );
        panels.insert(
            PanelType::QueryHistory,
            cx.new_view(|_| QueryHistoryPanel::new()).into(),
        );
        panels.insert(
            PanelType::Snippets,
            cx.new_view(|_| SnippetsPanel::new()).into(),
        );

        cx.new(|_cx| Self {
            active_panel: PanelType::Connections,
            width: px(280.0), // Default width
            is_collapsed: false,
            panels,
        })
    }

    pub fn toggle_panel(&mut self, panel_type: PanelType, cx: &mut Context<Self>) {
        if self.active_panel == panel_type && !self.is_collapsed {
            self.is_collapsed = true;
        } else {
            self.active_panel = panel_type;
            self.is_collapsed = false;
        }
        cx.notify();
    }

    pub fn toggle_collapse(&mut self, cx: &mut Context<Self>) {
        self.is_collapsed = !self.is_collapsed;
        cx.notify();
    }

    fn get_active_panel_title(&self) -> &str {
        match self.active_panel {
            PanelType::Connections => "Connections",
            PanelType::Schema => "Schema",
            PanelType::QueryHistory => "Query History",
            PanelType::Snippets => "Snippets",
        }
    }

    fn render_active_panel(&self, _cx: &mut Context<Self>) -> AnyElement {
        // Retrieve the active panel from the HashMap
        if let Some(panel) = self.panels.get(&self.active_panel) {
            panel.clone().into_any_element()
        } else {
            div()
                .p_3()
                .text_color(rgb(0xcccccc))
                .child("Panel not found")
                .into_any_element()
        }
    }

    fn render_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let connections_btn = self.render_panel_button(PanelType::Connections, "🗄", cx);
        let schema_btn = self.render_panel_button(PanelType::Schema, "📊", cx);
        let history_btn = self.render_panel_button(PanelType::QueryHistory, "🕐", cx);
        let snippets_btn = self.render_panel_button(PanelType::Snippets, "📝", cx);
        div()
            .flex()
            .items_center()
            .h(px(48.0))
            .border_t_1()
            .border_color(rgb(0x2d2d2d))
            .bg(rgb(0x252525))
            .children(connections_btn)
            .children(schema_btn)
            .children(history_btn)
            .children(snippets_btn)
    }

    fn render_panel_button(
        &self,
        panel_type: PanelType,
        icon: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_panel == panel_type && !self.is_collapsed;
        let button_id = format!("panel-button-{:?}", panel_type);

        div()
            .id(button_id)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .cursor_pointer()
            .when(is_active, |this| {
                this.bg(rgb(0x2d2d2d))
                    .border_t_2()
                    .border_color(rgb(0x007acc))
            })
            .hover(|this| this.bg(rgb(0x2a2a2a)))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.toggle_panel(panel_type, cx);
            }))
            .child(
                div()
                    .text_base()
                    .when(is_active, |this| this.text_color(rgb(0x007acc)))
                    .when(!is_active, |this| this.text_color(rgb(0x858585)))
                    .child(icon),
            )
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_content = if self.is_collapsed {
            div() // Render only the collapse/expand button when collapsed
                .flex()
                .flex_col()
                .h_full()
                .w(px(40.0)) // Fixed width for collapsed state
                .bg(rgb(0x1e1e1e))
                .border_r_1()
                .border_color(rgb(0x2d2d2d))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(|this| this.bg(rgb(0x2d2d2d)))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_collapse(cx);
                        }))
                        .child(
                            div().text_color(rgb(0xcccccc)).child("▶"), // Expand icon
                        ),
                )
        } else {
            div()
                .flex()
                .flex_col()
                .h_full()
                .w(self.width)
                .bg(rgb(0x1e1e1e))
                .border_r_1()
                .border_color(rgb(0x2d2d2d))
                .child(
                    // Header com título do painel ativo e botão de colapsar
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .h(px(40.0))
                        .px_3()
                        .border_b_1()
                        .border_color(rgb(0x2d2d2d))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xcccccc))
                                .child(self.get_active_panel_title()),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|this| this.bg(rgb(0x2d2d2d)))
                                .rounded_sm()
                                .p_1()
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.toggle_collapse(cx);
                                }))
                                .child(
                                    div().text_color(rgb(0xcccccc)).child("◀"), // Collapse icon
                                ),
                        ),
                )
                .child(
                    // Área de conteúdo do painel ativo
                    div()
                        .flex_1()
                        .overflow_y_scroll()
                        .child(self.render_active_panel(cx)),
                )
                .child(
                    // Footer com botões de navegação
                    self.render_footer(cx),
                )
        };

        div()
            .flex()
            .child(sidebar_content)
            .when(!self.is_collapsed, |this| {
                this.child(
                    // Resize Handle
                    div()
                        .w(px(6.0))
                        .h_full()
                        .cursor_col_resize()
                        .on_mouse_down(cx.listener(|this, event, _, cx| {
                            let start_x = event.position.x;
                            let initial_width = this.width;

                            // Capture mouse events for resizing
                            cx.on_mouse_move(window, |this, event, cx| {
                                let delta_x = event.position.x - start_x;
                                this.width =
                                    (initial_width + delta_x).max(px(100.0)).min(px(600.0)); // Min/Max width
                                cx.emit(SidebarWidthChanged(this.width)); // Emit event for external listeners
                                cx.notify();
                            })
                            .map(|_| ()) // map to () because on_mouse_move returns a future
                            .detach(); // Detach the listener when mouse button is released
                        }))
                        .bg(rgb(0x383838))
                        .hover(|this| this.bg(rgb(0x007acc))), // Highlight on hover
                )
            })
    }
}

// ============================================
// Exemplo de implementação: Painel de Conexões
// ============================================

pub struct ConnectionsPanel {
    connections: Vec<DatabaseConnection>,
}

#[derive(Clone)]
struct DatabaseConnection {
    name: String,
    db_type: String,
    host: String,
    is_connected: bool,
}

impl ConnectionsPanel {
    pub fn new() -> Self {
        Self {
            connections: vec![
                DatabaseConnection {
                    name: "Production DB".to_string(),
                    db_type: "PostgreSQL".to_string(),
                    host: "prod.example.com".to_string(),
                    is_connected: true,
                },
                DatabaseConnection {
                    name: "Development".to_string(),
                    db_type: "MySQL".to_string(),
                    host: "localhost:3306".to_string(),
                    is_connected: false,
                },
            ],
        }
    }

    fn render_connection(
        &self,
        conn: &DatabaseConnection,
        index: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let conn_id = format!("connection-{}", index);

        div()
            .id(conn_id.as_str())
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .rounded_md()
            .hover(|this| this.bg(rgb(0x2d2d2d)))
            .cursor_pointer()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        // Indicador de conexão
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .when(conn.is_connected, |this| this.bg(rgb(0x4ec9b0)))
                            .when(!conn.is_connected, |this| this.bg(rgb(0x858585))),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xcccccc))
                            .child(&conn.name),
                    )
                    .child(
                        div() // Actions for connection
                            .ml_auto()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x007acc))
                                    .cursor_pointer()
                                    .child("Edit")
                                    .on_click(cx.listener(move |_, _, cx| {
                                        println!("Edit connection: {}", conn.name);
                                    })),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xd16969))
                                    .cursor_pointer()
                                    .child("Delete")
                                    .on_click(cx.listener(move |_, _, cx| {
                                        println!("Delete connection: {}", conn.name);
                                    })),
                            ),
                    ),
            )
            .child(
                div()
                    .ml(px(20.0))
                    .text_xs()
                    .text_color(rgb(0x858585))
                    .child(format!("{} • {}", conn.db_type, conn.host)),
            )
    }
}

impl Render for ConnectionsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .child(
                div() // Add New Connection Button
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgb(0x007acc))
                    .text_color(rgb(0xffffff))
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|this| this.bg(rgb(0x005f99)))
                    .on_click(cx.listener(|_, _, _| {
                        println!("Add New Connection clicked!");
                    }))
                    .child("➕ New Connection"),
            )
            .children(
                self.connections
                    .iter()
                    .enumerate()
                    .map(|(i, conn)| self.render_connection(conn, i, cx)),
            )
    }
}

// ============================================
// Exemplo de implementação: Painel de Schema
// ============================================

pub struct SchemaPanel {
    tables: Vec<TableInfo>,
    expanded_tables: Vec<String>,
    filter_text: SharedString,
}

#[derive(Clone)]
struct TableInfo {
    name: String,
    row_count: u64,
    columns: Vec<ColumnInfo>,
}

#[derive(Clone)]
struct ColumnInfo {
    name: String,
    data_type: String,
    is_nullable: bool,
}

impl SchemaPanel {
    pub fn new() -> Self {
        Self {
            tables: vec![
                TableInfo {
                    name: "users".to_string(),
                    row_count: 15234,
                    columns: vec![
                        ColumnInfo {
                            name: "id".to_string(),
                            data_type: "INTEGER".to_string(),
                            is_nullable: false,
                        },
                        ColumnInfo {
                            name: "name".to_string(),
                            data_type: "VARCHAR(255)".to_string(),
                            is_nullable: false,
                        },
                        ColumnInfo {
                            name: "email".to_string(),
                            data_type: "VARCHAR(255)".to_string(),
                            is_nullable: true,
                        },
                    ],
                },
                TableInfo {
                    name: "orders".to_string(),
                    row_count: 89123,
                    columns: vec![
                        ColumnInfo {
                            name: "id".to_string(),
                            data_type: "INTEGER".to_string(),
                            is_nullable: false,
                        },
                        ColumnInfo {
                            name: "user_id".to_string(),
                            data_type: "INTEGER".to_string(),
                            is_nullable: false,
                        },
                        ColumnInfo {
                            name: "total".to_string(),
                            data_type: "DECIMAL(10,2)".to_string(),
                            is_nullable: false,
                        },
                    ],
                },
            ],
            expanded_tables: vec![],
            filter_text: SharedString::empty(),
        }
    }

    fn toggle_table(&mut self, table_name: &str, cx: &mut Context<Self>) {
        if let Some(pos) = self.expanded_tables.iter().position(|t| t == table_name) {
            self.expanded_tables.remove(pos);
        } else {
            self.expanded_tables.push(table_name.to_string());
        }
        cx.notify();
    }

    fn is_expanded(&self, table_name: &str) -> bool {
        self.expanded_tables.contains(&table_name.to_string())
    }

    fn render_table(&self, table: &TableInfo, cx: &mut Context<Self>) -> impl IntoElement {
        let is_expanded = self.is_expanded(&table.name);
        let table_name = table.name.clone();
        let table_id = format!("table-{}", table.name);

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(table_id.as_str())
                    .flex()
                    .items_center()
                    .gap_2()
                    .p_2()
                    .rounded_md()
                    .hover(|this| this.bg(rgb(0x2d2d2d)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_table(&table_name, cx);
                    }))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x858585))
                            .child(if is_expanded { "▼" } else { "▶" }),
                    )
                    .child(div().text_sm().text_color(rgb(0xcccccc)).child(&table.name))
                    .child(
                        div()
                            .ml_auto()
                            .text_xs()
                            .text_color(rgb(0x858585))
                            .child(format!("{} rows", table.row_count)),
                    ),
            )
            .when(is_expanded, |this| {
                this.child(div().ml(px(24.0)).flex().flex_col().gap_px().children(
                    table.columns.iter().map(|col| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .p_1()
                            .pl_2()
                            .rounded_sm()
                            .hover(|this| this.bg(rgb(0x2d2d2d)))
                            .child(div().text_xs().text_color(rgb(0x9cdcfe)).child(&col.name))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x858585))
                                    .child(&col.data_type),
                            )
                            .when(!col.is_nullable, |this| {
                                this.child(
                                    div().text_xs().text_color(rgb(0xd16969)).child("NOT NULL"),
                                )
                            })
                    }),
                ))
            })
    }
}

impl Render for SchemaPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let filtered_tables = self
            .tables
            .iter()
            .filter(|table| table.name.contains(&self.filter_text.to_string()))
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .child(
                div() // Filter input
                    .h(px(32.0))
                    .mb_2()
                    .px_2()
                    .rounded_md()
                    .bg(rgb(0x2d2d2d))
                    .text_color(rgb(0xcccccc))
                    .child("🔍 Filter (not yet functional)"), // Placeholder for a real input
            )
            .children(
                filtered_tables
                    .into_iter()
                    .map(|table| self.render_table(table, cx)),
            )
    }
}

// ============================================
// Novo: Painel de Histórico de Queries
// ============================================

pub struct QueryHistoryPanel;

impl QueryHistoryPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for QueryHistoryPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_3()
            .text_color(rgb(0xcccccc))
            .child("Query History - Coming soon")
    }
}

// ============================================
// Novo: Painel de Snippets
// ============================================

pub struct SnippetsPanel;

impl SnippetsPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for SnippetsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_3()
            .text_color(rgb(0xcccccc))
            .child("Snippets - Coming soon")
    }
}
