

use edsa_pos::{sale::{
    accounts::{TransactionIn, OutTransaction},
    people::{Employee, Person, Sex, Role}, 
    inventory::{FinishedProduct, DailyYield, Product, RawMaterial}}, 
    fetch_transaction_in_log, 
    fetch_transaction_out_log, 
    fetch_employee_logs, 
    fetch_people_logs, 
    fetch_finished_product_log, 
    fetch_daily_logs, fetch_raw_material_log
};

use serde::{Serialize, Deserialize};
use eframe::{
    epi::App, 
    egui::{
        CentralPanel, TopBottomPanel, self, 
        TextStyle, Label, Layout, CtxRef, 
        RichText, Color32, Button, 
        Separator, Visuals, SidePanel, 
        FontDefinitions, FontFamily, 
        Sense, Align, panel::Side, Window, ScrollArea, Ui
    }
};

#[allow(dead_code)]
pub struct TempEnums {
    sex: Sex,
    role: Role,
}
impl Default for TempEnums {
    fn default() -> Self {
        Self { sex: Sex::Male, role: Role::Customer }
    }
}

#[derive(Default,Clone)]
pub struct TempVecs {
    a: Vec<Employee>,
    b: Vec<Person>,
    c: Vec<TransactionIn>,
    d: Vec<OutTransaction>,
}

#[derive(Serialize,Deserialize)]
pub struct Config {
    //theme
    dark_mode: bool,
    //main control
    main_active: bool,
    //main
    inv_win: bool,
    cash_win: bool,
    staff_win: bool,
    pips_win: bool,
    //cash_win
    cash_t_win: bool,
    cash_d_win: bool,
    //cash_t_win
    buy_window: bool,
    sell_window: bool,
    // cash_d_win
    ext_debts: bool,
    int_debts: bool,
    //add daily yield
    daily_win: bool
}
impl Default for Config{
    fn default() -> Self {
        Self { 
            dark_mode: false, 
            inv_win: true, 
            cash_win: false, 
            staff_win: false, 
            pips_win: false, 
            cash_t_win: true, 
            cash_d_win: false, 
            buy_window: false, 
            sell_window: true,
            main_active: false,
            daily_win: false,
            ext_debts: true,
            int_debts: false, 
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    money_in: Vec<TransactionIn>,
    money_out: Vec<OutTransaction>,
    staff: Vec<Employee>,
    sup_cus: Vec<Person>,
    fin_prod: FinishedProduct,
    raw_mat: RawMaterial,
    daily_rec: Vec<DailyYield>,
}

impl Default for Package {
    fn default() -> Self {
        let money_in = fetch_transaction_in_log().unwrap();
        let money_out = fetch_transaction_out_log().unwrap();
        let staff = fetch_employee_logs().unwrap();
        let sup_cus = fetch_people_logs().unwrap();
        let fin_prod = fetch_finished_product_log().unwrap();
        let raw_mat = fetch_raw_material_log().unwrap();
        let daily_rec = fetch_daily_logs().unwrap();

        Self { money_in, money_out, staff, sup_cus, fin_prod, raw_mat, daily_rec }
    }
}
#[allow(unused_variables)]
#[derive(Debug)]
pub struct Editor {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: bool,
    j: bool,
}
impl Editor {
    pub fn new() -> Self {
        Self { a:String::default(), 
            b:String::default(), 
            c:String::default(), 
            d:String::default(), 
            e:String::default(), 
            f:String::from("0"), 
            g:String::from("0"), 
            h:String::from("0"), 
            i: false, 
            j: false 
        } 
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct State {
    package: Package,
    config: Config,
    editor: Editor,
    vecs: TempVecs,
    enums: TempEnums,
}

impl State {
    pub fn new() -> Self {
        let package = Package::default();
        let config :Config = confy::load("edsafeeds").unwrap_or_default();
        let editor = Editor::default();
        let vecs = TempVecs::default();
        let enums = TempEnums::default();
        Self { package, config, editor, vecs, enums } 
    }
    
    pub fn render_staff_list(&self, ui: &mut Ui) {
        let color = if self.config.dark_mode{ Color32::WHITE }else{ Color32::BLACK};
        for person in &self.package.staff {
            ui.colored_label(color,RichText::new(&person.name));
            ui.add_space(8.);
            ui.label(RichText::new(format!("☎ {:?}",&person.tel)));
            ui.add_space(8.);
            ui.add(Separator::default());
        }
    }

    pub fn buy_window(&mut self, ui: &mut Ui) {
        // selling -> TransanctionIn
        ui.columns(2, |col| {
            // col[1].set_style(crate::styles::get_style());
            col[1].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[0].label(RichText::new("Buy Window").underline().strong());
            col[0].add_space(10.);

            col[0].horizontal(|ui|{
                ui.label("supplier"); ui.add_space(5.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.i = false; 
                    self.editor.j = true;
                }
                let add = ui.button("add new"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.j = false;
                    self.editor.i = true;                        }
            });
            col[0].add_space(10.);

            col[0].label("Maize quantity(kg)");
            col[0].add_space(5.);
            col[0].text_edit_singleline(&mut self.editor.f);
            col[0].add_space(10.);

            col[0].label("price per kg");
            col[0].add_space(5.);
            col[0].text_edit_singleline(&mut self.editor.g);


            if self.editor.i {
                col[1].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.i = false;
                        self.editor.j = false;
                    }
                    ui.separator();
                    ui.label("Add Supplier");
                });
                col[1].separator();
                col[1].label("name");
                col[1].add_space(5.);
                col[1].text_edit_singleline(&mut self.editor.c);
                col[1].add_space(10.);

                col[1].label("tel no.");
                col[1].add_space(5.);
                col[1].text_edit_singleline(&mut self.editor.d);
                col[1].add_space(10.);

                if col[1].button("add").clicked() {
                    if self.editor.c.len()>1 && self.editor.d.len()>1 {
                        let name = self.editor.c.clone();
                        let tel = self.editor.d.clone();
                        let role = Role::Supplier;

                        let p = Person::new(role, name, tel);
                        // add to current vector
                        self.package.sup_cus.push(p.clone());
                        // log
                        p.log().unwrap();
                        // clean up
                        self.editor.c = String::default();
                        self.editor.d = String::default();
                        self.editor.i = false;
                    }
                }
                col[1].add_space(10.);
                col[1].separator();
            }
            if self.editor.j {
                col[1].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.i = false;
                        self.editor.j = false;
                    }
                    ui.separator();
                    let help_text = RichText::new("type to narrow search").color(Color32::LIGHT_BLUE);
                    ui.label("Search").on_hover_text(help_text);
                   
                });

                col[1].add_space(5.);
                let search = col[1].text_edit_singleline(&mut self.editor.a);
                col[1].add_space(5.);
                col[1].separator();

                if search.changed() {
                    let p: Vec<_> = self.package.sup_cus.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.a)
                    }).collect();
                    self.vecs.b = p;
                }
                ScrollArea::vertical().show(&mut col[1], |ui|{
                    for  sp in self.vecs.b.iter(){
                        ui.label(&sp.name);
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(5.);
                        if ui.button("pick").clicked(){
                            for (i, so) in self.package.sup_cus.iter().enumerate() {
                                if sp == so {
                                    self.editor.h = i.to_string();
                                }
                            }
                            //clean up
                            self.editor.j = false;
                        };
                        ui.separator();
                    }
                });
            }
            if !self.editor.i && !self.editor.j {
                col[1].indent("main", |ui|{
                    ui.label(RichText::new("TRANSACTION DETAILS"));
                    ui.add_space(20.);
                    ui.label("supplier");
    
                    if self.package.sup_cus.len()>0{
                        ui.indent("space5", |ui|{
                            if let Ok(index) = self.editor.h.parse::<usize>(){
                                ui.label(RichText::new(&self.package.sup_cus[index].name));
                                ui.label(RichText::new(&self.package.sup_cus[index].tel) );
                            }
                        });
                    }else{
                        ui.label("no one yet");
                    }  
                    
                    let qty = self.editor.f.parse::<u32>();
                    let price = self.editor.g.parse::<u32>();
                    if let Ok(e) = qty{
                        if let Ok(f) = price{
                            ui.add_space(20.);
                            let commentary = RichText::new(format!("-> {} kg(s) of flour at {} per kg", e, f));
                            ui.label(commentary);
                            ui.add_space(7.);
                            /*
                            will need to add error handling for type overflow **
                            */
                            ui.label(RichText::new(format!("total cost: {}",e*f)));
                            ui.add_space(20.);
                            ui.horizontal(|ui|{
                                ui.label("amount paid: ");
                                ui.text_edit_singleline(&mut self.editor.e);
                            });
    
                            // complete the transaction
                            if let Ok(_index) = self.editor.h.parse::<usize>(){
                                ui.add_space(7.);
                                let finish = ui.button("complete transanction");
    
                                if finish.clicked() {
                                    if e > 0 && f > 0 {
                                        if let Ok(paid) = self.editor.e.parse::<u32>() {
                                            let supplier = if let Ok(index) = self.editor.h.parse::<usize>() {
                                                    self.package.sup_cus[index].to_owned()
                                                }else{ Person::default() };
    
                                            let item = RawMaterial::new("Maize".to_string(), e);
                                            let mut trans = OutTransaction::new(supplier, item);
    
                                            trans.update(f);
    
                                            trans.settle_bill(paid);
                                            self.package.money_out.push(trans.to_owned());
                                            trans.balance_books(&mut self.package.raw_mat);
                                            trans.log();
                                            self.editor = Editor::default();
                                        }
                                    }
                                }
                            }
                        }
                    }

                });
            }
        });     
    }
    
    pub fn sell_window(&mut self, ui: &mut Ui){
        // selling -> TransanctionIn
        ui.columns(2, |col| {
            col[1].visuals_mut().override_text_color = Some(Color32::DARK_BLUE);
            col[0].label(RichText::new("Sell Window").underline().strong());
            col[0].add_space(10.);

            col[0].horizontal(|ui|{
                ui.label("buyer"); ui.add_space(5.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.i = false; 
                    self.editor.j = true;
                }
                let add = ui.button("add new"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.j = false;
                    self.editor.i = true;                        }
            });
            col[0].add_space(10.);

            col[0].label("Flour quantity(kg)");
            col[0].add_space(5.);
            col[0].text_edit_singleline(&mut self.editor.f);
            col[0].add_space(10.);

            col[0].label("price per kg");
            col[0].add_space(5.);
            col[0].text_edit_singleline(&mut self.editor.g);

            if self.editor.i {
                col[1].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.i = false;
                        self.editor.j = false;
                    }
                    ui.separator();
                    ui.label("Add Buyer");
                });
                col[1].label("name");
                col[1].add_space(5.);
                col[1].text_edit_singleline(&mut self.editor.c);
                col[1].add_space(10.);

                col[1].label("tel no.");
                col[1].add_space(5.);
                col[1].text_edit_singleline(&mut self.editor.d);
                col[1].add_space(10.);

                if col[1].button("add").clicked() {

                    if self.editor.c.len()>1 && self.editor.d.len()>1 {
                        let name = self.editor.c.clone();
                        let tel = self.editor.d.clone();
                        let role = Role::Customer;

                        let p = Person::new(role, name, tel);
                        // add to current vector
                        self.package.sup_cus.push(p.clone());
                        // log
                        p.log().unwrap();
                        // clean up
                        self.editor.c = String::default();
                        self.editor.d = String::default();
                        self.editor.i = false;
                    }
                }
                col[1].add_space(10.);
                col[1].separator();
            }
            if self.editor.j {
                col[1].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.i = false;
                        self.editor.j = false;
                    }
                    ui.separator();
                    ui.label("Search").on_hover_text("type to narrow search");
                });
                col[1].add_space(5.);
                let search = col[1].text_edit_singleline(&mut self.editor.a);
                col[1].add_space(5.);
                col[1].separator();

                if search.changed() {
                    let p: Vec<_> = self.package.sup_cus.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.a)
                    }).collect();
                    self.vecs.b = p;
                }
                ScrollArea::vertical().show(&mut col[1], |ui|{
                    for  sp in self.vecs.b.iter(){
                        ui.label(&sp.name);
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(5.);
                        if ui.button("pick").clicked(){
                            for (i, so) in self.package.sup_cus.iter().enumerate() {
                                if sp == so {
                                    self.editor.h = i.to_string();
                                }
                            }
                            //clean up
                            self.editor.j = false;
                        };
                        ui.separator();
                    }
                });
            }
            if !self.editor.i && !self.editor.j {
                col[1].indent("main2", |col|{
                    col.label(RichText::new("TRANSACTION DETAILS"));
                    col.add_space(20.);
                    col.label("buyer");
                    if self.package.sup_cus.len()>0{
                        col.indent("space4", |ui|{
                            if let Ok(index) = self.editor.h.parse::<usize>(){
                                ui.label(RichText::new(&self.package.sup_cus[index].name));
                                ui.label(RichText::new(&self.package.sup_cus[index].tel));
                            }
                        });
                    }else{
                        col.label("no one yet");
                    }  
                    
                    let qty = self.editor.f.parse::<u32>();
                    let price = self.editor.g.parse::<u32>();
                    if let Ok(e) = qty{
                        if let Ok(f) = price{
                            col.add_space(20.);
                            let commentary = RichText::new(format!("->{}kg(s) of flour at {} per kg", e, f));
                            col.label(commentary);
                            col.add_space(7.);
                            /*
                            will need to add error handling for type overflow **
                            */
                            col.label(RichText::new(format!("total cost: {}",e*f)));
                            
                            col.add_space(20.);
                            col.horizontal(|ui|{
                                ui.label("amount paid:");
                                ui.text_edit_singleline(&mut self.editor.e);
                            });
    
                            // complete the transaction
                            if let Ok(_index) = self.editor.h.parse::<usize>(){
                                col.add_space(7.);
                                let finish = col.button("complete transanction");
                                if finish.clicked() {
                                    if e > 0 && f > 0 {
                                        if let Ok(paid) = self.editor.e.parse::<u32>() {
                                            let buyer = if let Ok(index) = self.editor.h.parse::<usize>() {
                                                self.package.sup_cus[index].to_owned()
                                            }else{ Person::default() }; //else maybe
                                            let mut trans = TransactionIn::new(buyer);
                                            let item = Product::new("Flour".to_string(), None, f, e);
                                            trans.add_item(item);
                                            trans.settle_bill(paid);
                                            self.package.money_in.push(trans.to_owned());
                                            trans.balance_books(&mut self.package.fin_prod);
                                            trans.log();
                                            self.editor = Editor::default();
                                        }
                                    }
                                }
                            }
    
                        }
                    }
                });
            }
        });     
    }
    
    pub fn render_top_panel(&mut self, ctx: &CtxRef){
        let frame = crate::styles::get_frame();

        TopBottomPanel::top("header").frame(frame)
         .show(ctx, |ui|{
            ui.add_space(5.);
            egui::menu::bar(ui, |ui|{
                ui.with_layout(Layout::left_to_right(), |ui|{
                    let inv = ui.add(Button::new(RichText::new("Inventory")
                            .color(if self.config.inv_win { Color32::GREEN }else{Color32::DARK_BLUE})
                            .monospace().strong().text_style(TextStyle::Body) 
                        ) );
                    if inv.clicked(){
                        println!("inventory");
                        self.config.inv_win = true;
                        self.config.cash_win = false;
                        self.config.staff_win = false;
                        self.config.pips_win = false;
                    }
                    ui.add(Separator::default());

                    let cash = ui.add(Button::new(RichText::new("Cash")
                            .color(if self.config.cash_win { Color32::GREEN }else{ Color32::DARK_BLUE })
                            .monospace().strong().text_style(TextStyle::Body)
                        ) );
                    if cash.clicked(){
                        println!("MoneyMan");
                        self.config.inv_win = false;
                        self.config.cash_win = true;
                        self.config.staff_win = false;
                        self.config.pips_win = false;
                    }
                    ui.add(Separator::default());

                    let staff = ui.add(Button::new(RichText::new("Staff")
                            .color(if self.config.staff_win { Color32::GREEN } else { Color32::DARK_BLUE }) 
                            .monospace().strong().text_style(TextStyle::Body) 
                        ) );
                    if staff.clicked(){
                        println!("Bees");
                        self.config.inv_win = false;
                        self.config.cash_win = false;
                        self.config.staff_win = true;
                        self.config.pips_win = false;
                    }    
                    ui.add(Separator::default());

                    let pips = ui.add(Label::new(RichText::new("Clientele")
                            .color(if self.config.pips_win { Color32::GREEN }else{Color32::DARK_BLUE})
                            .text_style(TextStyle::Body) 
                        ).sense(Sense::click()) );
                    if pips.clicked(){
                        println!("RainMen");
                        self.config.inv_win = false;
                        self.config.cash_win = false;
                        self.config.staff_win = false;
                        self.config.pips_win = true;
                    }
                    ui.add(Separator::default());
                });
                ui.with_layout(Layout::right_to_left(),|ui|{
                    let theme_btn=ui.add(Button::new(RichText::new("🔆").text_style(TextStyle::Body))) ;
                    if theme_btn.clicked(){
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
            ui.add_space(5.);
        });
    }
    
    pub fn render_inventory_win(&mut self, ctx: &CtxRef){
        // let frame = crate::styles::get_frame();
        CentralPanel::default().show(ctx, |ui|{
            egui::menu::bar(ui, |ui|{

                ui.with_layout(Layout::left_to_right(),|ui|{
                    let previous =ui.add(Button::new(RichText::new("◀").heading())) ;
                    ui.add(Separator::default());
                    ui.label(RichText::new("Daily Yield").heading());
                });

                ui.with_layout(Layout::right_to_left(),|ui|{
                    let next =ui.add(Button::new(RichText::new("▶").heading())) ;
                    ui.add(Separator::default());
                    if !self.package.daily_rec.is_empty(){
                        let i = self.package.daily_rec.len() - 1;
                        ui.label(RichText::new(
                            format!(
                                "{}-{}-{}",
                                self.package.daily_rec[i].date.day,
                                self.package.daily_rec[i].date.month,
                                self.package.daily_rec[i].date.year 
                            )
                        ).heading());
                    }
                });

            });
        
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui|{
                let i = self.package.daily_rec.len() - 1;
                ui.label(RichText::new( self.package.daily_rec[i].quantity.to_string() ).heading());
                ui.add_space(20.);
                if ui.button(RichText::new("➕")).clicked() {
                    self.config.daily_win = true;
                    self.config.inv_win = false;
                }; 
            });

            ui.add_space(20.);
            ui.add(Separator::default());
            ui.add_space(20.);

            ui.columns(2, |col|{
                col[0].with_layout(Layout::top_down_justified(Align::Center), |ui|{
                    ui.label(RichText::new("total available product").heading());
                    ui.add_space(20.);
                    ui.label(RichText::new(format!("{}kg",self.package.fin_prod.available_quantity)).heading());
                });
                col[1].with_layout(Layout::top_down_justified(Align::Center), |ui|{
                    ui.label(RichText::new("total available raw material").heading());
                    ui.add_space(20.);
                    ui.label(RichText::new(format!("{}kg",self.package.raw_mat.available_quantity)).heading());
                });
            });
        });
        
    }

    pub fn daily_window( &mut self, ctx: &CtxRef ) {
        let frame = crate::styles::get_frame();
        Window::new("Add to daily yield (kg)").default_width(400.).frame(frame)
         .show(ctx, |ui|{
           
            ui.add_space(15.);
            ui.horizontal(|ui|{
                ui.label("Quantity :");
                ui.add_space(5.);
                ui.text_edit_singleline(&mut self.editor.f);
            });
            ui.add_space(15.);
            ui.horizontal(|ui|{
                ui.add_space(300.);
                let close = ui.button(RichText::new("Close").monospace().strong().text_style(TextStyle::Body));
                let add =  ui.button(RichText::new("Add").monospace().strong().text_style(TextStyle::Body));
                if add.clicked() {
                    if let Ok(qty) = self.editor.f.parse::<u32>() {
                        if qty > 0{
                            let product = String::from("Flour");
                            let quantity = qty;
                            let stock = &mut self.package.fin_prod;
        
                            let dy = DailyYield::new(product, quantity, stock);
                            self.package.daily_rec.push(dy);
        
                            self.config.daily_win = false;
                            self.config.inv_win = true;
                            // self.config.main_active = true;
                        }
                    }
                }
                if close.clicked() {
                    self.config.daily_win = false;
                    self.config.inv_win = true;
                }
            });
            ui.add_space(10.)
        
        });
    }

    pub fn render_cash_win(&mut self, ctx: &CtxRef){
        let frame = crate::styles::get_frame();
        let sp = SidePanel::new(Side::Left, "side_menu").min_width(130.).max_width(130.).frame(frame);
        sp.show(ctx, |ui|{
            ui.add_space(10.);
            let trans = ui.add(Label::new(RichText::new("transactions")
              .color(if self.config.cash_t_win {Color32::DARK_RED} else {Color32::BLACK })
              .strong() ).sense(Sense::click()));
            if trans.clicked(){
                self.config.cash_d_win=false;
                self.config.cash_t_win=true;
            }
            ui.add(Separator::default().spacing(10.) );
            let dt = ui.add(Label::new(RichText::new("debts")
              .color(if self.config.cash_d_win {Color32::DARK_RED} else {Color32::BLACK  })
              .strong()).sense(Sense::click()));
            if dt.clicked(){
                self.config.cash_d_win=true;
                self.config.cash_t_win=false;
            }
            ui.add(Separator::default().spacing(10.) );
        });
        
        if self.config.cash_t_win {
            CentralPanel::default().show(ctx, |ui|{
                egui::menu::bar(ui, |ui|{
          
                    let rt = RichText::new("buy").color(if self.config.buy_window{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let rst = RichText::new("sell").color(if self.config.sell_window{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let ls = ui.button(rst.heading()); 
                    ui.separator();
                    let lb = ui.button(rt.heading());
                    ui.separator();

                    if lb.clicked() {
                        self.config.buy_window = true;
                        self.config.sell_window = false;
                    }
                    if ls.clicked() {
                        self.config.sell_window = true;
                        self.config.buy_window = false;
                    }
                });
                if self.config.buy_window {
                    ui.add_space(20.);
                    self.buy_window(ui);
                }else if self.config.sell_window {
                    ui.add_space(20.);
                    self.sell_window(ui);
                }
                
            });

        }else if self.config.cash_d_win {
            CentralPanel::default().show(ctx, |ui|{
                egui::menu::bar(ui, |ui|{
          
                    let rt = RichText::new("External Debts").color(if self.config.ext_debts{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let rst = RichText::new("Internal Debts").color(if self.config.int_debts{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let lb = ui.button(rt.heading());
                    ui.separator();
                    let ls = ui.button(rst.heading()); 
                    ui.separator();

                    if lb.clicked() {
                        self.config.ext_debts = true;
                        self.config.int_debts = false;
                    }
                    if ls.clicked() {
                        self.config.int_debts = true;
                        self.config.ext_debts = false;
                    }
                });
                if self.config.ext_debts {
                    ui.add_space(20.);
                    self.external_debts(ui);
                }else if self.config.int_debts {
                    ui.add_space(20.);
                    self.internal_debts(ui);
                }
            });
        }
    }
    
    pub fn render_staff_win(&mut self, ctx: &CtxRef){
        let rsp = SidePanel::new(Side::Right, "right_pane").min_width(400.);
        rsp.show(ctx, |ui|{
            ScrollArea::vertical().show(ui, |ui|{
                self.render_staff_list(ui);
            });
        });

        CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui|{
                ui.add_space(20.);
                ui.label(RichText::new("Name"));
                ui.add_space(20.);
                let _text_input = ui.text_edit_singleline( &mut self.editor.a );
            }); 
            ui.add_space(20.);

            ui.horizontal(|ui|{
                ui.add_space(20.);
                ui.label(RichText::new("Tel No."));
                // ui.add_space(20.);
                let _text_input = ui.text_edit_singleline( &mut self.editor.b );
            }); 
            ui.add_space(20.);

            ui.horizontal(|ui|{
                ui.add_space(20.);
                ui.checkbox(&mut self.editor.i, RichText::new("active").text_style(TextStyle::Body));
            });
            ui.add_space(20.);

            ui.horizontal(|ui|{
                ui.add_space(20.);
                ui.label("Sex: ");
                ui.selectable_value(&mut self.enums.sex , Sex::Male, RichText::new("Male").text_style(TextStyle::Body) );
                ui.separator();
                ui.selectable_value(&mut self.enums.sex, Sex::Female, RichText::new("Female").text_style(TextStyle::Body));
            });
            ui.add_space(20.);

            ui.horizontal(|ui|{
                ui.add_space(20.);
                ui.with_layout(Layout::right_to_left(), |ui|{
                    if ui.button(RichText::new("add").text_style(TextStyle::Body)).clicked(){
                        if self.editor.a.len()>1 && self.editor.b.len()>1 {
                            let name = self.editor.a.clone();
                            let tel = self.editor.b.clone();
                            let active = self.editor.i;
                            let sex = self.enums.sex;
    
                            let p = Employee::new(name, sex, active, tel);
    
                            self.package.staff.push(p.clone());
                            p.log().expect("couldnt log.. sorry..");
                            self.editor.a = String::default();
                            self.editor.b = String::default();
                        } 
                    }
                })
            });
            ui.add(Separator::default());

            ui.label("Search");
            let search = ui.text_edit_singleline(&mut self.editor.c);
            
            if search.changed() {
                let p: Vec<_> = self.package.staff.to_owned().into_iter().filter(|emp|{
                    emp.name.contains(&self.editor.c)
                }).collect();
                println!("{:?}",p);
                self.vecs.a = p;
            }
            ScrollArea::vertical().show(ui, |ui|{
                for sp in self.vecs.a.iter(){
                    ui.label(&sp.name);
                    ui.add_space(5.);
                    ui.label(&sp.tel);
                    ui.separator();
                }
            });

        });
    }
    
    pub fn render_pips_win(&mut self, ctx: &CtxRef){
        CentralPanel::default().show(ctx, |ui| {
            ui.set_style(crate::styles::get_style());

            ui.label("text");
            ui.button("a button");
            ui.label("text");
            ui.button("a button");
        });
    }
    
    pub fn external_debts(&mut self, ui: &mut Ui) {
        // properties:
        // all incomplete transactions
        // filter by person -> settle all method
        //                  -> settle a single transaction
        //                  -> partly settle the debt
        ui.columns(3,|col|{
            col[0].label(RichText::new("Unaowadai"));
            col[0].separator();

            ScrollArea::vertical().show(&mut col[0], |ui|{
                ui.label("aaaaa");
            });
        });

    }

    fn internal_debts(&mut self, ui: &mut Ui) {
        ui.label("lets begun");
    }
}


impl State {
    pub fn configure_fonts(&self, ctx: &CtxRef){
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "Roboto".to_owned(),
            egui::FontData::from_static(include_bytes!("/home/klan/edsa/edsafeeds/fonts/roboto/Roboto-Bold.ttf")),
        );
        font_def.family_and_size.insert(
            TextStyle::Body, 
            (FontFamily::Monospace, 20.),
        );
        font_def.family_and_size.insert(
            TextStyle::Heading, 
            (FontFamily::Monospace, 30.),
        );
        // Put my font first (highest priority)
        font_def.fonts_for_family.get_mut(
            &FontFamily::Proportional).unwrap().insert(0, "Roboto".to_owned()
        ); 
        ctx.set_fonts(font_def);
    }
}


impl App for State {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        if self.config.dark_mode{
            ctx.set_visuals(Visuals::dark());
        }else{
            ctx.set_visuals(crate::styles::light());
        }

        self.render_top_panel(ctx);

        if self.config.inv_win{
            self.render_inventory_win(ctx);
        }else if self.config.cash_win {
            self.render_cash_win(ctx);
        }else if self.config.pips_win {
            self.render_pips_win(ctx);
        }else if self.config.staff_win {
            self.render_staff_win(ctx);
        }else if self.config.daily_win {
            self.daily_window(ctx)
        }
    }

    fn name(&self) -> &str {
        "EDSA FEEDS"
    }

    fn setup(&mut self, ctx: &egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        self.configure_fonts(ctx);
    }
}
