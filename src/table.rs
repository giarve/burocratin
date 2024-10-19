use std::{sync::Arc, usize::MAX};

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use web_sys::HtmlElement;

use crate::{
    css::TABLE_ROW,
    data::Aeat720Record,
    utils::{
        icons::{render_svg_delete_square_icon, render_svg_edit_icon, render_svg_save_icon},
        usize_to_date,
    },
};

#[derive(Debug, PartialEq, Clone)]
struct Aeat720RecordInfo {
    record: Aeat720Record,
    editable: bool,
}
pub struct Table {
    headers: Vec<&'static str>,
    data: MutableVec<Mutable<Aeat720RecordInfo>>,
}

impl Table {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            headers: vec![
                "Nombre compañía",
                "ISIN",
                "Código país",
                "Fecha 1ª adquisición",
                "Valor Euros",
                "Nº acciones",
                "Porcentaje",
            ],
            data: MutableVec::new(),
        })
    }

    pub fn table_rows_not_empty(&self) -> impl Signal<Item = bool> {
        self.data
            .signal_vec_cloned()
            .to_signal_map(|x| !x.is_empty())
    }

    pub fn extend_rows(&self, records: Vec<Aeat720Record>) {
        for record in records.into_iter() {
            self.data
                .lock_mut()
                .push_cloned(Mutable::new(Aeat720RecordInfo {
                    record,
                    editable: false,
                }));
        }
    }

    pub fn get_records(&self) -> Vec<Aeat720Record> {
        let mut result = vec![];
        for record in self.data.lock_ref().iter() {
            result.push(record.lock_ref().record.clone());
        }
        result
    }

    pub fn clear(&self) {
        self.data.lock_mut().clear();
    }

    fn render_header_cells(this: &Arc<Self>) -> Vec<Dom> {
        this.headers
            .iter()
            .map(|header_cell| {
                html!("th", {
                  .attr("scope", "col")
                  .style("vertical-align", "bottom")
                  .style("font-weight", "bold")
                  .style("background-color", "#ddd")
                  .text(header_cell)
                })
            })
            .collect()
    }

    fn render_header(this: &Arc<Self>) -> Dom {
        html!("thead", {
          .child(
            html!("tr", {
              .child(
                html!("th", {
                  .attr("scope", "col")
                  .style("vertical-align", "bottom")
                  .style("font-weight", "bold")
                  .style("background-color", "#ddd")
                  .text("")
                })
              )
              .children(Self::render_header_cells(this))
              .child(
                html!("th", {
                  .attr("scope", "col")
                  .style("vertical-align", "bottom")
                  .style("font-weight", "bold")
                  .style("background-color", "#ddd")
                  .text("")
                })
              )
            })
          )
        })
    }

    fn render_row(this: &Arc<Self>, index: usize, record: &Mutable<Aeat720RecordInfo>) -> Dom {
        let date = usize_to_date(record.lock_ref().record.first_tx_date)
            .map_or("".to_string(), |d| d.format("%d/%m/%Y").to_string());

        let action_span = html!("span" => HtmlElement, {
          .child(render_svg_edit_icon("red", "24"))
          .with_node!(_element => {
            .event(clone!(record => move |_: events::Click| {
              record.lock_mut().editable = true;
            }))
          })
        });

        let delete_span = html!("span" => HtmlElement, {
          .child(render_svg_delete_square_icon("red", "24"))
          .with_node!(_element => {
            .event(clone!(this => move |_: events::Click| {
              this.data.lock_mut().remove(index);
            }))
          })
        });

        html!("tr", {
          .class(&*TABLE_ROW)
          .children(&mut [
           html!("td", {
              .text(&format!("{}", index + 1))
            }),
           html!("td", {
              .text(&record.lock_ref().record.company.name)
            }),
            html!("td", {
              .text(&record.lock_ref().record.company.isin)
            }),
            html!("td", {
              .text(&record.lock_ref().record.broker.country_code)
            }),
            html!("td", {
              .text(&date)
            }),
            html!("td", {
              .text(&record.lock_ref().record.value_in_euro.to_string())
            }),
            html!("td", {
              .text(&record.lock_ref().record.quantity.to_string())
            }),
            html!("td", {
              .text(&record.lock_ref().record.percentage.to_string())
              .text("%")
            }),
            html!("td", {
              .child(action_span)
              .child(delete_span)
            }),
          ])
        })
    }

    fn render_editable_row(
        this: &Arc<Self>,
        index: usize,
        record: &Mutable<Aeat720RecordInfo>,
    ) -> Dom {
        let date = usize_to_date(record.lock_ref().record.first_tx_date)
            .map_or("".to_string(), |d| d.format("%d/%m/%Y").to_string());

        let action_span = html!("span" => HtmlElement, {
          .child(render_svg_save_icon("red", "24"))
          .with_node!(_element => {
            .event(clone!(record => move |_: events::Click| {
              record.lock_mut().editable = false;
            }))
          })
        });

        let delete_span = html!("span" => HtmlElement, {
          .child(render_svg_delete_square_icon("red", "24"))
          .with_node!(_element => {
            .event(clone!(this => move |_: events::Click| {
              this.data.lock_mut().remove(index);
            }))
          })
        });

        html!("tr", {
          .class(&*TABLE_ROW)
          .children(&mut [
           html!("td", {
              .text(&format!("{}", index + 1))
            }),
           html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.company.name)
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.company.isin)
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.broker.country_code)
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &date)
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.value_in_euro.to_string())
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.quantity.to_string())
              }))
            }),
            html!("td", {
              .child(html!("input", {
                .attr("type", "text")
                .attr("value", &record.lock_ref().record.percentage.to_string())
              }))
              .text("%")
            }),
            html!("td", {
              .child(action_span)
              .child(delete_span)
            }),
          ])
        })
    }

    fn render_body(this: &Arc<Self>) -> Dom {
        html!("tbody", {
          .children_signal_vec(this.data.signal_vec_cloned()
            .enumerate().map(clone!(this => move |(index, record)| {
              let i = index.get().unwrap_or(usize::MAX);
              if record.lock_ref().editable {
                Table::render_editable_row(&this, i, &record)
              } else {
                Table::render_row(&this, i, &record)
              }
            }))
          )
        })
    }

    fn is_needed_to_rerender_rows(this: &Arc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            // let _editable_changed = this.editable.signal(),
            let _records_len = this.data.signal_vec_cloned().to_signal_map(|x| x.len()) =>
            true
        }
    }

    pub fn render(this: &Arc<Self>) -> Dom {
        html!("table", {
          .style("overflow", "auto")
          .style("width", "100%")
          .style("height", "400px")
          .style("border-collapse", "collapse")
          .style("border", "1px solid #8c8c8c")
          .style("margin-bottom" ,"1em")
          .child(
            html!("caption", {
              .text("Movimientos importados.")
            })

          )
          .child(Self::render_header(this))
          .child_signal(Self::is_needed_to_rerender_rows(this).map(
            clone!(this => move |_x| {
              Some(Self::render_body(&this))
            }))
          )
        })
    }
}
