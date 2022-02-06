
use std::{path::Path, fs, io::Write};

use eframe::{
    epi::App, egui::{
        self, CtxRef, TopBottomPanel, 
        Layout, Button, RichText, 
        Separator, Label, Color32, 
        Sense, FontDefinitions, TextStyle, 
        FontFamily, SidePanel, panel::Side, 
        CentralPanel, Ui, ScrollArea, Window 
    }
};

use edsa_pos::{
    pipeline::{
        accounts::{
            Debtor, Creditor, OutTransaction, TransactionIn
        }, 
        inventory::{
            FinishedProd, RawMaterial, PackagedProd, Product, Production, DailyYield
        }, 
        people::{
            Person, Employee, Sex
        }
    }, 
    fetch_logs, PathOption, LogPartial, fetch_daily_logs
};

use crate::styles::top_panel_frame;


/*********Setting Up***********/
#[derive(Clone)]
pub struct TempVecs {
    rm: Vec<RawMaterial>,
    item_list: Vec<String>,
    actual_item_list: Vec<RawMaterial>,
    pip: Vec<Person>,
    pip_actual: Vec<Person>,
    pkg: Vec<PackagedProd>,
    actual_pkg_list: Vec<PackagedProd>,
    dy: Vec<Vec<DailyYield>>,
    prod: Vec<Product>,
    production: Vec<Production>,
    prod_actual: Vec<Product>,
    debt: Vec<Debtor>,
    credit: Vec<Creditor>,
    in_trans: Vec<TransactionIn>,
    out_trans: Vec<OutTransaction>,
    // and temp indices.. haha!!
    index: usize,
    fp_index: usize,
    pkg_index: usize,
    p_index: usize,
    // temp enum..
    sex: Sex,

}
impl Default for TempVecs {
    fn default() -> Self {
        let mut dy: Vec<Vec<DailyYield>> = Vec::new();
        let p_list = fetch_logs::<Product>(PathOption::Product).unwrap();
        for pr in p_list {
            let path_str = format!("records/{}dyield",&pr.name);
            let path = Path::new(&path_str);
            let list = fetch_daily_logs(path).unwrap();
            // dbg!(&list);
            let _ = &dy.push(list);
        }
        let mut in_trans = fetch_logs::<TransactionIn>(PathOption::TransIn).unwrap();
        in_trans.reverse();
        
        let mut out_trans = fetch_logs::<OutTransaction>(PathOption::TransOut).unwrap();
        out_trans.reverse();
        
        let mut production = fetch_logs::<Production>(PathOption::Production).unwrap();
        production.reverse();

        let pip_actual: Vec<Person> = Vec::new();
        Self { 
            rm: Default::default(), 
            item_list: Default::default(), 
            actual_item_list: Default::default(), 
            pip: Default::default(), 
            pip_actual,
            pkg: Default::default(),
            actual_pkg_list: Default::default(),
            dy,
            prod: Default::default(),
            production,
            prod_actual: Default::default(),
            index: 0,
            p_index: 0,
            fp_index: 0,
            pkg_index: 0,
            debt: Default::default(),
            credit: Default::default(),
            in_trans,
            out_trans,
            sex: Sex::Male,
        }
    }
}

pub struct PkgLists {
    money_in: Vec<TransactionIn>,
    money_out: Vec<OutTransaction>,
    staff: Vec<Employee>,
    people: Vec<Person>,
    product: Vec<Product>,
    production: Vec<Production>,
    fin_prod: Vec<FinishedProd>,
    pkg_prod: Vec<PackagedProd>,
    raw_mat: Vec<RawMaterial>,
    debtors: Vec<Debtor>,
    creditors: Vec<Creditor>,   
}
impl Default for PkgLists {
    fn default() -> Self {
        let money_in = fetch_logs::<TransactionIn>(PathOption::TransIn).unwrap();
        let money_out = fetch_logs::<OutTransaction>(PathOption::TransOut).unwrap();
        let staff = fetch_logs::<Employee>(PathOption::Staff).unwrap();
        let people = fetch_logs::<Person>(PathOption::People).unwrap();
        let fin_prod = fetch_logs::<FinishedProd>(PathOption::FinProd).unwrap();
        let product = fetch_logs::<Product>(PathOption::Product).unwrap();
        let production = fetch_logs::<Production>(PathOption::Production).unwrap();
        let pkg_prod = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
        let raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
        let debtors = fetch_logs::<Debtor>(PathOption::Debtors).unwrap();
        let creditors = fetch_logs::<Creditor>(PathOption::Creditor).unwrap();

        Self { money_in, money_out, staff, people, fin_prod, product, production, pkg_prod, raw_mat, debtors, creditors, /*dyield*/ }
    }
}

pub struct Editor {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: String,
    j: String,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
    p: bool,
    q: bool,
    r: bool,
    s: bool,
    z: bool,
    y: bool,
}
impl Editor {
    pub fn new() -> Self {
        Self { 
            a:String::from(""), 
            b:String::from(""), 
            c:String::from(""), 
            d:String::from(""), 
            e:String::from(""), 
            f:String::from(""), 
            g:String::from("0"), 
            h:String::from("0"), 
            i:String::from("0"), 
            j:String::from("0"),
            k:false,
            l:false,
            m:false,
            n:false,
            o:false, 
            p:false, 
            q:false,
            r:false,
            s:false, 
            z:false,
            y:false,
        } 
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Default)]
pub struct Config {
    win_config: WindowConfig,
    sale_config: SaleConfig,
    sale_normal_config: SaleNormalConfig,
    sale_debt_config: SaleDebtConfig,
    misc_pops: MiscPopWins,
}

pub struct WindowConfig {
    sales_win: bool,
    inventory_win: bool,
    logs_win: bool,
    staff_win: bool,
}
impl Default for WindowConfig {
    fn default() -> Self {
        Self { sales_win: false, inventory_win: true, logs_win: false, staff_win: false }
    }
}
pub struct SaleConfig {
    normal_win: bool,
    debt_win: bool,
}
impl Default for SaleConfig {
    fn default() -> Self {
       Self{normal_win: true, debt_win: false}
    }
}
pub struct SaleNormalConfig {
    buy_win: bool,
    sell_win: bool,
}
impl Default for SaleNormalConfig {
    fn default() -> Self {
        Self {buy_win: false, sell_win: true }
    }
}
pub struct SaleDebtConfig {
    debtors: bool,
    creditors: bool,
}
impl Default for SaleDebtConfig {
    fn default() -> Self {
        Self { debtors: true, creditors: false }
    }
}
pub struct MiscPopWins {
    edit_rawmat: bool,
    edit_pkgprod: bool, 
}
impl Default for MiscPopWins {
    fn default() -> Self {
        Self { edit_rawmat: false, edit_pkgprod: false }
    }
}



/************START**************/

pub struct State {
    apk: PkgLists,
    conf: Config,
    editor: Editor,
    tvecs: TempVecs,
}

impl Default for State {
    fn default() -> Self {
        Self { apk: PkgLists::default(), conf: Default::default(), editor: Editor::default(), tvecs: TempVecs::default() }
    }
}

impl App for State {
    fn setup(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        // will configure fonts here
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        
        self.render_top_panel(ctx);
        if self.conf.misc_pops.edit_rawmat {
            self.edit_rawmat(ctx);
        }
        if self.conf.misc_pops.edit_pkgprod {
            self.edit_pkgprod(ctx);
        }
        if self.conf.win_config.inventory_win {
            self.render_inventory_win(ctx);
        }
        if self.conf.win_config.sales_win {
            self.render_sales_win(ctx);
        }
        if self.conf.win_config.logs_win {
            self.render_logs_win(ctx);
        }
        if self.conf.win_config.staff_win {
            self.render_staff_win(ctx);
        }
    }

    fn name(&self) -> &str {
        "Edsa Feeds"
    }
}

impl State {
    pub fn render_top_panel(&mut self, ctx: &CtxRef) {
        // let fnt = crate::styles::font_def();
        // ctx.set_fonts(fnt);
        let frame = top_panel_frame();

        TopBottomPanel::top("header").frame(frame)
         .show(ctx, |ui|{

            ui.set_style(crate::styles::top_panel_style());

            ui.add_space(5.);
            egui::menu::bar(ui, |ui|{
                ui.with_layout(Layout::left_to_right(), |ui|{
                    let inv = ui.add(Button::new( RichText::new("Inventory")
                      .strong().monospace().heading().color(
                        if self.conf.win_config.inventory_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
                    ) ));
                    if inv.clicked(){
                        self.conf.win_config.inventory_win = true;
                        self.conf.win_config.sales_win = false;
                        self.conf.win_config.logs_win = false;
                        self.conf.win_config.staff_win = false;

                        self.conf.misc_pops.edit_pkgprod = false;
                        self.conf.misc_pops.edit_rawmat = false;

                        self.editor = Editor::default();
                    }
                    ui.add(Separator::default());

                    let cash = ui.add(Button::new(RichText::new("Cash")
                      .strong().monospace().heading().heading().color(
                        if self.conf.win_config.sales_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
                    )  ));
                    if cash.clicked(){
                        self.conf.win_config.inventory_win = false;
                        self.conf.win_config.sales_win = true;
                        self.conf.win_config.logs_win = false;
                        self.conf.win_config.staff_win = false;

                        self.editor = Editor::default();
                    }
                    ui.add(Separator::default());

                    let staff = ui.add(Button::new(RichText::new("Staff")
                      .strong().monospace().heading().color(
                        if self.conf.win_config.staff_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
                    ) ));
                    if staff.clicked(){
                        self.conf.win_config.inventory_win = false;
                        self.conf.win_config.sales_win = false;
                        self.conf.win_config.logs_win = false;
                        self.conf.win_config.staff_win = true;
                    }   
                    ui.add(Separator::default());

                    let pips = ui.add(Button::new(RichText::new("Logs")
                      .strong().monospace().heading().color(
                        if self.conf.win_config.logs_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
                    ) ));
                    if pips.clicked(){
                        self.conf.win_config.inventory_win = false;
                        self.conf.win_config.sales_win = false;
                        self.conf.win_config.staff_win = false;
                        self.conf.win_config.logs_win = true;
                    }
                    ui.add(Separator::default());
                });
                // ui.with_layout(Layout::right_to_left(),|ui|{
                //     ui.add_space(10.);
                //     let theme_btn=ui.add(Button::new(RichText::new("🔆"))) ;
                //     if theme_btn.clicked() {}
                // });
            });
            ui.add_space(5.);
        });
    }  

    pub fn buy_window(&mut self, ui: &mut Ui) {
               
        ui.columns(3, |col| {
            col[2].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[0].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            
            col[1].set_style(crate::styles::top_panel_style());

            col[1].label(RichText::new("Buy Window"));
            col[1].separator();
            col[1].horizontal(|ui|{
                ui.label(RichText::new("supplier").color(Color32::BLACK)); ui.add_space(15.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.k = false; 
                    self.editor.l = true;
                }
                let add = ui.button("add new ➕"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.l = false;
                    self.editor.k = true;
                }
            });
            col[1].add_space(10.);

            if !self.tvecs.pip_actual.is_empty() {
                col[1].label(RichText::new(&self.tvecs.pip_actual[0].name));
                let tel = format!("☎: {}",&self.tvecs.pip_actual[0].tel);
                col[1].label(RichText::new(&tel));
                col[1].separator();
            }

            col[1].horizontal(|ui|{
                ui.label(RichText::new("items' list").color(Color32::BLACK));
                ui.add_space(15.);
                if ui.button(RichText::new("add item ➕")).clicked() {
                    self.editor.n = true;
                };
            });
            col[1].add_space(10.);
            // col[1].separator();
            ScrollArea::vertical().id_source("raw_scroll").max_height(280.)
              .show(&mut col[1], |ui|{

                let mut index: usize = 0; 
                for (i, item) in self.tvecs.actual_item_list.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&item.name));
                        ui.separator();
                        if ui.button(RichText::new("❎")).clicked() {
                            index = i;
                            self.editor.o = true;
                        }
                    });
                    ui.add_space(5.);
                    ui.horizontal(|ui|{
                        let qty = format!("quantity: {}kg",item.quantity);
                        ui.label(RichText::new(&qty));
                        if let Some(price) = item.price_per{
                            let price = format!("price: Ksh.{}",price);
                            ui.separator();
                            ui.label(RichText::new(&price));
                        }
                    });
                    ui.add_space(5.);
                    ui.separator();
                }
                if self.editor.o {
                    self.tvecs.actual_item_list.remove(index);
                    self.editor.o = false;
                }
            });
            if self.tvecs.actual_item_list.len() > 0 { // ***************************************
                let total_cost: f32 = self.tvecs.actual_item_list.iter()
                    .map(|item| item.quantity * item.price_per.unwrap()).sum();
                let tc = format!("total cost: {}",total_cost);
                col[1].add_space(15.);
                col[1].label(RichText::new(tc.to_string()));
            }

            col[1].horizontal(|ui| {
                ui.label(RichText::new("Settle Bill: "));
                ui.text_edit_singleline(&mut self.editor.j);
            });

            if col[1].button("complete purchase ✅").clicked() {
                if !self.tvecs.pip_actual.is_empty() && !self.tvecs.actual_item_list.is_empty() {
                    let per = self.tvecs.pip_actual[0].to_owned();
                    let mut tr = OutTransaction::new(per);
                    for rm in &self.tvecs.actual_item_list {
                        tr.add(rm.to_owned());

                    }
                    // settle bill and log
                    if let Ok(bs) = self.editor.j.parse::<f32>() {
                        tr.settle_bill(bs);
                        tr.balance_books();
                        self.apk.money_out.push(tr.to_owned());
                        let path = Path::new("records/out_acc");
                        tr.log(path);
                        // reset the temp lists
                        self.tvecs.item_list = Vec::new();
                        self.tvecs.actual_item_list = Vec::new();
                        self.tvecs.pip = Vec::new();
                        self.tvecs.pip_actual = Vec::new();
                        self.editor.j = String::default();
                        //re order material list
                        self.apk.raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
                        self.apk.creditors = fetch_logs::<Creditor>(PathOption::Creditor).unwrap();
                        // transaction list
                        let mut list = self.apk.money_out.clone();
                        list.reverse();
                        self.tvecs.out_trans = list;
                    }
                    // dbg!(tr);
                }
            }
            // A recent Transaction list *********************************************
            if self.tvecs.actual_item_list.len() <= 2 {
                col[1].add_space(7.);
                col[1].vertical_centered(|ui|{
                    ui.label(RichText::new("Recent Transactions").underline());
                });
                col[1].add_space(7.);

                ScrollArea::vertical().id_source("rec_trans_out")
                .show(&mut col[1], |ui|{
                    for (i,tr) in self.tvecs.out_trans.iter().enumerate() {
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new(&tr.person.name));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.label(RichText::new(&tr.time).color(Color32::DARK_BLUE));
                            });
                        });
                        ui.label(RichText::new(format!("total cost: {}",tr.total_cost)).color(Color32::DARK_GRAY));
                        if let Some(balance) = tr.balance {
                            ui.label(RichText::new(format!("unpaid balance: {}",balance)).color(Color32::RED));
                        }else {
                            ui.label(RichText::new(format!("unpaid balance: NONE ")).color(Color32::DARK_GREEN));
                        }
                        ui.separator();

                        if i == 4 { break; }
                    }
                });
            }

            col[1].add_space(10.);
            
            
            if self.editor.k {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.f = String::from("");
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                    ui.separator();
                    ui.label("Add Person");
                });
                col[0].label("name");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(10.);

                col[0].label("tel no.");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.e);
                col[0].add_space(10.);

                if col[0].button("add").clicked() {
                    if self.editor.e.len()>1 && self.editor.f.len()>1 {
                        let name = self.editor.f.clone();
                        let tel = self.editor.e.clone();

                        let p = Person::new(name, tel);
                        // add to current vector
                        self.apk.people.push(p.clone());
                        // log
                        let path = std::path::Path::new("records/people");
                        p.log(path);
                        // clean up
                        self.editor.e = String::default();
                        // self.editor.f = String::default();
                        // self.editor.i = false;
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                }
                col[0].add_space(10.);
                col[0].separator();
            }
            
            if self.editor.l {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = true;
                        self.editor.l = false;
                    }
                    ui.separator();
                    ui.label("Search").on_hover_text("type to narrow search");
                });
                col[0].add_space(5.);
                let search = col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(5.);
                col[0].separator();

                if search.changed() {
                    let p: Vec<_> = self.apk.people.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.f)
                    }).collect();
                    self.tvecs.pip = p;
                }

                ScrollArea::vertical().show(&mut col[0], |ui|{
                    for  sp in self.tvecs.pip.iter(){
                        ui.label(RichText::new(&sp.name).color(Color32::BLACK));
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(3.);
                        ui.horizontal(|ui|{
                            ui.add_space(7.);
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                if ui.button("pick").clicked(){
                                    self.tvecs.pip_actual = Vec::new();
                                    self.tvecs.pip_actual.push(sp.to_owned());
                                };
                            });
                        });
                        ui.separator();
                    }
                });
            }

            // add item
            if self.editor.n {
                col[2].add_space(30.);
                col[2].horizontal(|ui|{
                    ui.label(RichText::new("register a new raw material"));
                    if ui.button(RichText::new("add new")).clicked() {
                        self.editor.m = true;
                        self.editor.n = false;
                    }
                });

                col[2].separator();
                col[2].horizontal( |ui| {
                    ui.label(RichText::new("search"));
                    let search = ui.text_edit_singleline(&mut self.editor.a);
                    if search.changed() {
                        let p: Vec<_> = self.apk.raw_mat.to_owned().into_iter().filter(|rm|{
                            rm.name.contains(&self.editor.a)
                        }).collect();
                        dbg!(&p);
                        self.tvecs.rm = p;
                    }
                });
                col[2].separator();
    
                ScrollArea::vertical().id_source("search_scroll")
                  .show(&mut col[2], |ui|{
                    for rm in self.tvecs.rm.iter() {
                        ui.label(RichText::new(&rm.name).color(Color32::BLACK));
                        ui.add_space(5.);
                        let tstr = format!("remaining quantity: {}",&rm.quantity);
                        ui.label(RichText::new(tstr));
                        ui.add_space(3.);
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.add_space(5.);
                                if ui.button("pick").clicked() {
                                    // add to list // pop a window
                                    self.tvecs.item_list.push(rm.name.to_owned());
                                    dbg!(&self.tvecs.item_list);
                                    self.conf.misc_pops.edit_rawmat = true;
                                }
                            });
                        });
                        ui.add_space(5.);
                        ui.separator();
                    }
                });
            }
            // add new raw material
            if self.editor.m {
                col[2].add_space(30.);
                col[2].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.b = String::from("");
                            self.editor.g = 0.to_string();
                            self.editor.m = false;
                            self.editor.n = true;
                    }
                    ui.separator();
                });

                col[2].label(RichText::new("type name below.."));
                col[2].add_space(7.);
                col[2].text_edit_singleline(&mut self.editor.b);
                col[2].add_space(7.);
                col[2].label(RichText::new("quantity in store already"));
                col[2].text_edit_singleline(&mut self.editor.g);
                
                if col[2].button(RichText::new("add")).clicked() {
                    if let Ok(g) = self.editor.g.parse::<u32>() {
                        if self.editor.b.len() > 2 {
                            RawMaterial::new( self.editor.b.to_owned(), g as f32 ).local_log().unwrap();
                            self.apk.raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
                            self.editor.b = String::from("");
                            self.editor.g = 0.to_string();
                            self.editor.m = false;
                            self.editor.n = true;
                        }
                    }
                }
            }
        });   
    }

    pub fn edit_rawmat(&mut self, ctx: &CtxRef) {
        let frame = top_panel_frame();
        Window::new("edit Raw Material").min_width(400.).frame(frame)
        .show(ctx, |ui|{
            let i = self.tvecs.item_list.len()-1;
            let name = &self.tvecs.item_list[i];
            ui.label(RichText::new(&*name));
            ui.horizontal(|ui|{
                ui.label(RichText::new("quantity:      "));
                ui.text_edit_singleline(&mut self.editor.h);
            });
            ui.horizontal(|ui|{
                ui.label(RichText::new("price offered:"));
                ui.text_edit_singleline(&mut self.editor.i);
            });
            ui.horizontal(|ui|{
                ui.add_space(300.);
                if ui.button(RichText::new("confirm")).clicked() {
                    if let Ok(qty) = self.editor.h.parse::<f32>() {
                        if let Ok(price) = self.editor.i.parse::<u32>() {
                            let mut item = RawMaterial::new(name.to_owned(), qty);
                            item.price(price);
                            self.tvecs.actual_item_list.push(item);
                            // and then the finisher
                            self.conf.misc_pops.edit_rawmat = false;
                        }
                    }
                }
                ui.add_space(5.);
                if ui.button(RichText::new("close")).clicked() {
                    self.conf.misc_pops.edit_rawmat = false;
                }
                ui.add_space(15.);
            });
        });
    }

    pub fn sell_window(&mut self, ui: &mut Ui) {
               
        ui.columns(3, |col| {
            col[2].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[0].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[1].set_style(crate::styles::top_panel_style());
            

            col[1].label(RichText::new("Sell Window"));
            col[1].separator();
            col[1].horizontal(|ui|{
                ui.label(RichText::new("Buyer").color(Color32::BLACK)); ui.add_space(15.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.k = false; 
                    self.editor.l = true;
                }
                let add = ui.button("add new ➕"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.l = false;
                    self.editor.k = true;
                }
            });
            col[1].add_space(10.);

            if !self.tvecs.pip_actual.is_empty() {
                col[1].label(RichText::new(&self.tvecs.pip_actual[0].name));
                let tel = format!("☎: {}",&self.tvecs.pip_actual[0].tel);
                col[1].label(RichText::new(&tel));
                col[1].separator();
            }

            col[1].horizontal(|ui|{
                ui.label(RichText::new("items' list").color(Color32::BLACK));
                ui.add_space(15.);
                if ui.button(RichText::new("add item ➕")).clicked() {
                    self.editor.n = true;
                };
            });
            col[1].add_space(10.);
            // col[1].separator();
            ScrollArea::vertical().id_source("pkg_s_scroll").max_height(280.)
              .show(&mut col[1], |ui|{

                let mut index: usize = 0; 
                for (i, item) in self.tvecs.actual_pkg_list.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&item.pkg_specify));
                        let prinfo = format!("{}, {}kg",item.product.name, item.quantity);
                        ui.label(RichText::new(prinfo).strong().small());
                        ui.separator();
                        if ui.button(RichText::new("❎")).clicked() {
                            index = i;
                            self.editor.o = true;
                        }
                    });
                    ui.add_space(5.);
                    ui.horizontal(|ui|{
                        let qty = format!("amount: {} packs",item.total);
                        let price = format!("price: Ksh.{} each",item.cost);
                        ui.label(RichText::new(&qty));
                        ui.separator();
                        ui.label(RichText::new(&price));
                    });
                    ui.add_space(5.);
                    ui.separator();
                }
                if self.editor.o {
                    self.tvecs.actual_pkg_list.remove(index);
                    self.editor.o = false;
                }
            });
            let total_cost: u32 = self.tvecs.actual_pkg_list.iter()
                .map(|item| item.total * item.cost as u32).sum();
            let tc = format!("total cost: {}",total_cost);
            col[1].add_space(15.);
            col[1].label(RichText::new(tc.to_string()));

            col[1].horizontal(|ui| {
                ui.label(RichText::new("Settle Bill: "));
                ui.text_edit_singleline(&mut self.editor.j);
            });

            if col[1].button("complete purchase ✅").clicked() {
                if !self.tvecs.pip_actual.is_empty() && !self.tvecs.actual_pkg_list.is_empty() {

                    let per = self.tvecs.pip_actual[0].to_owned();
                    let mut tr = TransactionIn::new(per);
                    
                    for pkg in &self.tvecs.actual_pkg_list {
                        tr.add(pkg.to_owned());
                    }
                    // balance book, settle bill and log
                    if let Ok(bs) = self.editor.j.parse::<f32>() {
                        tr.balance_books();// logs the pkg prod
                        tr.settle_bill(bs);
                        let path = Path::new("records/in_acc");
                        tr.log(path);
                        dbg!(&tr);
                        self.apk.money_in.push(tr);
                        // more clean up
                        self.apk.pkg_prod = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
                        self.apk.debtors = fetch_logs::<Debtor>(PathOption::Debtors).unwrap();
                        // reset the temp lists
                        self.tvecs.item_list = Vec::new();
                        self.tvecs.actual_pkg_list = Vec::new();
                        self.tvecs.pip = Vec::new();
                        self.tvecs.pip_actual = Vec::new();
                        self.editor.j = String::default();
                         // transaction list
                        let mut list = self.apk.money_in.clone();
                        list.reverse();
                        self.tvecs.in_trans = list;
                    }
                }
            }
            if self.tvecs.actual_pkg_list.len() <= 2 {
                col[1].add_space(7.);
                col[1].vertical_centered(|ui|{
                    ui.label(RichText::new("Recent Transactions").underline());
                }); 
                col[1].add_space(7.);
                ScrollArea::vertical().id_source("rec_trans")
                .show(&mut col[1], |ui|{
                    for (i, tr) in self.tvecs.in_trans.iter().enumerate() {
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new(&tr.person.name));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.label(RichText::new(&tr.time).color(Color32::DARK_BLUE));
                            });
                        });
                        ui.label(RichText::new(format!("total cost: {}",tr.total_cost)).color(Color32::DARK_GRAY));
                        if let Some(balance) = tr.balance {
                            ui.label(RichText::new(format!("unpaid balance: {}",balance)).color(Color32::RED));
                        }else {
                            ui.label(RichText::new(format!("unpaid balance: NONE ")).color(Color32::DARK_GREEN));
                        }
                        ui.separator();

                        if i == 4 { break; }
                    }
                });
            }

            col[1].add_space(10.);
            
            
            if self.editor.k {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.f = String::from("");
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                    ui.separator();
                    ui.label("Add Person");
                });
                col[0].label("name");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(10.);

                col[0].label("tel no.");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.e);
                col[0].add_space(10.);

                if col[0].button("add").clicked() {
                    if self.editor.e.len()>2 && self.editor.f.len()>1 {
                        let name = self.editor.f.clone();
                        let tel = self.editor.e.clone();

                        let p = Person::new(name, tel);
                        // add to current vector
                        self.apk.people.push(p.clone());
                        // log
                        let path = std::path::Path::new("records/people");
                        p.log(path);
                        // clean up
                        self.editor.e = String::default();
                        // self.editor.f = String::default();
                        // self.editor.i = false;
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                }
                col[0].add_space(10.);
                col[0].separator();
            }
            
            if self.editor.l {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = true;
                        self.editor.l = false;
                    }
                    ui.separator();
                    ui.label("Search").on_hover_text("type to narrow search");
                });
                col[0].add_space(5.);
                let search = col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(5.);
                col[0].separator();

                if search.changed() {
                    let p: Vec<_> = self.apk.people.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.f)
                    }).collect();
                    self.tvecs.pip = p;
                }
                
                ScrollArea::vertical().show(&mut col[0], |ui|{
                    for  sp in self.tvecs.pip.iter(){
                        ui.label(RichText::new(&sp.name).color(Color32::BLACK));
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(3.);
                        ui.horizontal(|ui|{
                            ui.add_space(7.);
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                if ui.button("pick").clicked(){
                                    self.tvecs.pip_actual = Vec::new();
                                    self.tvecs.pip_actual.push(sp.to_owned());
                                };
                            });
                        });
                        ui.separator();
                    }
                });
            }

            // add item
            if self.editor.n {
                col[2].add_space(30.);

                col[2].separator();
                col[2].horizontal( |ui| {
                    ui.label(RichText::new("search"));
                    let search = ui.text_edit_singleline(&mut self.editor.a);
                    if search.changed() {
                        let p: Vec<_> = self.apk.pkg_prod.to_owned().into_iter().filter(|pkg|{
                            pkg.pkg_specify.contains(&self.editor.a)
                        }).collect();
                        self.tvecs.pkg = p;
                    }
                });
                col[2].separator();
    
                ScrollArea::vertical().id_source("search_scroll")
                  .show(&mut col[2], |ui|{
                    for pkg in self.tvecs.pkg.iter() {
                        ui.label(RichText::new(&pkg.pkg_specify).color(Color32::BLACK));
                        ui.add_space(5.);
                        let tstr = format!("in stock: {} packs",&pkg.total);
                        ui.label(RichText::new(tstr));
                        ui.add_space(3.);
                        ui.horizontal(|ui|{
                            ui.add_space(5.);
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                if ui.button("pick").clicked() {
                                    // add to list // pop a window
                                    self.tvecs.item_list.push(pkg.pkg_specify.to_owned());
                                    self.conf.misc_pops.edit_pkgprod = true;
                                }
                            });
                        });
                        ui.add_space(5.);
                        ui.separator();
                    }
                });
            }
            // *****start here********
        });
    }

    pub fn edit_pkgprod(&mut self, ctx: &CtxRef) {
        let frame = top_panel_frame();
        Window::new("edit Brand purchase").min_width(400.).frame(frame)
        .show(ctx, |ui|{
            let i = self.tvecs.item_list.len()-1;
            let name = &self.tvecs.item_list[i];
            ui.label(RichText::new(&*name));

            ui.horizontal(|ui|{
                ui.label(RichText::new("no. of packs : "));
                ui.text_edit_singleline(&mut self.editor.h);
            });

            ui.horizontal(|ui|{
                ui.add_space(290.);
                if ui.button(RichText::new("confirm")).clicked() {
                    if let Ok(qty) = self.editor.h.parse::<u32>() {
                        if let Some((mut pp, no)) = PackagedProd::sell_pkg(name.to_owned()) {
                            if qty <= no && qty != 0 {
                                pp.add_packs(qty);
                                self.tvecs.actual_pkg_list.push(pp);
                                self.conf.misc_pops.edit_pkgprod = false;
                            }
                        }
                    }
                }
                ui.add_space(5.);
                if ui.button(RichText::new("close")).clicked() {
                    self.conf.misc_pops.edit_pkgprod = false;
                }
                ui.add_space(15.);
            });
        });
    }

    pub fn render_inventory_win(&mut self, ctx: &CtxRef) {
        let frame = crate::styles::top_panel_frame();

        SidePanel::new(Side::Left, "left_side").min_width(300.).max_width(300.).frame(frame)
        .show(ctx, |ui| {
            ui.set_style(crate::styles::top_panel_style());
            // add a product ///////////////////////////////////////
            ui.label(RichText::new("add a new product"));
            ui.text_edit_singleline(&mut self.editor.a);
            ui.add_space(5.);
            if ui.button("add").clicked() {
                if self.editor.a.len()>2 {
                    let pr = Product::new(self.editor.a.to_owned());

                    // a new product should automatically allocate store space
                    let fp = FinishedProd::new(pr.to_owned());
                    let path2 = Path::new("records/finprod");
                    self.apk.fin_prod.push(fp.to_owned());
                    fp.log(path2);

                    // and a default daily yield to start off
                    let pr2 = pr.clone();

                    // update local list
                    self.apk.product.push(pr.to_owned());
                    // log
                    let path = Path::new("records/products");
                    pr.log(path);

                    // and a default daily yield to start off
                    DailyYield::new(pr2, 0.0);
                    self.editor.a = String::default();
                }
            }
            ui.separator();
            // /////////////////////////////////////////////////////
            // normal window
            if !self.editor.k && !self.editor.r && !self.editor.s && !self.editor.z {
                // ////////////////////////////////////////////////////////////////////////////
                ui.label(RichText::new("In Stock(Unpacked)").strong().underline());
                ui.add_space(10.);
                ScrollArea::vertical().id_source("finprod").max_height(200.)
                .show(ui, |ui|{
                    for (i, fp) in self.apk.fin_prod.iter().enumerate() {
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new(&fp.product.name));
                                let tstr = format!("{} kg",&fp.quantity);
                                ui.label(RichText::new(&tstr).color(Color32::BROWN));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.add_space(5.);
                                if ui.button(RichText::new("add")).clicked() {
                                    // add finidhed product.. coming from production ***********
                                    self.editor.r = true;
                                    self.tvecs.fp_index = i;
                                }
                            });
                        });
                        ui.add_space(5.);
                    }
                });
                ui.separator();
                // /////////////////////////////////////////////////////////////////////////////
                ui.add_space(20.);
                // /////////////////////////////////////////////////////////////////////
                ui.horizontal_top(|ui| {
                    ui.add_space(10.);
                    ui.label(RichText::new("Packaged Products").strong().underline());
                
                    ui.add_space(15.);
                    if ui.button(RichText::new("new brand ➕ ")).clicked() {
                        // add a new brand ***************************
                        self.editor.k = true;
                        self.tvecs.item_list = Vec::new();
                    }
                });
                ui.add_space(10.);
                
                ScrollArea::vertical().id_source("pkgprod")//.max_height(100.)
                .show(ui, |ui|{
                    for (i, pkg) in self.apk.pkg_prod.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new((i+1).to_string()));
                                // ui.separator();
                                ui.label(RichText::new(&pkg.pkg_specify));
                                // ui.add_space(10.);
                                let fstr = format!("{} packs",&pkg.total);
                                ui.label(RichText::new(fstr).color(Color32::GRAY));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                // edit price here *********************************************************************
                                ui.add_space(2.);
                                if ui.button(RichText::new("Add")).clicked() {
                                    self.tvecs.pkg_index = i;
                                    self.editor.s = true;
                                }
                                ui.add_space(2.);
                                if ui.button(RichText::new("Edit")).clicked() {
                                    self.tvecs.pkg_index = i;
                                    self.editor.z = true;
                                }
                            });
                        });
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.add_space(2.);
                                let f = format!("{}, {}kg, Ksh{}",&pkg.product.name, &pkg.quantity, pkg.cost as u32);
                                ui.label(RichText::new(&f));
                            });
                        });
                        ui.separator();
                    }
                });
                // //////////////////////////////////////////////////////////////////////////
            }
            // adding a new packaged product
            if self.editor.k {
                ui.label(RichText::new("New Brand"));
                ui.add_space(5.);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("product"));
                    if ui.button(RichText::new("choose")).clicked() {
                        self.editor.l = true;
                        self.tvecs.item_list = Vec::new();
                    }
                    if !self.tvecs.item_list.is_empty() {
                        ui.label(RichText::new(&self.tvecs.item_list[0] ));
                    }
                });
                // pause
                if self.editor.l {
                    ScrollArea::vertical().id_source("productscroll").max_height(100.)
                    .show(ui, |ui|{
                        ui.label(RichText::new("products"));
                        for (i, pr) in self.apk.product.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new((i+1).to_string()));
                                ui.label(RichText::new(&pr.name));
                                if ui.button(RichText::new("pick")).clicked() {
                                    let _ = &self.tvecs.item_list.push(pr.name.to_owned());
                                    self.editor.l = false;
                                }
                            });
                        }
                    });
                }
                // continue
                ui.horizontal(|ui| {
                    ui.label(RichText::new("name:   "));
                    ui.text_edit_singleline(&mut self.editor.b);
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("quantity:"));
                    ui.text_edit_singleline(&mut self.editor.j);
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("price:   "));
                    ui.text_edit_singleline(&mut self.editor.i);
                });
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("Add")).clicked() {
                        if let Ok(qty) = self.editor.j.parse::<f32>() {
                            if let Ok(price) = self.editor.i.parse::<u32>() {
                                if qty > 0.0 && !self.tvecs.item_list.is_empty() {
                                    
                                    let prd = Product::new(self.tvecs.item_list[0].to_owned() );
                                    let mut pkg = PackagedProd::new(prd);

                                    pkg.specify_cost(price as f32);
                                    pkg.specify_pkg(self.editor.b.to_owned());
                                    pkg.specify_qty(qty);
                                    self.apk.pkg_prod.push(pkg.to_owned());

                                    let path = Path::new("records/pkgprod");
                                    pkg.log(path);

                                    self.editor.k = false;
                                }
                            }
                        }
                    }
                    ui.add_space(5.);
                    if ui.button(RichText::new("Close")).clicked() {
                        self.editor.k = false;
                    }
                });
            }
            // adding to stock (Dailyyield)
            if self.editor.r {
                let sftr = format!("add {} stock:", self.apk.fin_prod[self.tvecs.fp_index].product.name);
                ui.label(RichText::new(&sftr));
                ui.text_edit_singleline(&mut self.editor.i);
                ui.horizontal(|ui|{
                    ui.with_layout(Layout::right_to_left(),|ui|{
                        ui.add_space(15.);
                        if ui.button(RichText::new("Close")).clicked() { 
                            self.editor.r = false;
                        }
                        if ui.button(RichText::new("Done")).clicked() {
                            if let Ok(qty) = self.editor.i.parse::<f32>() {
                                // this will-> update finprod log and daily log
                                let _ = DailyYield::new(self.apk.fin_prod[self.tvecs.fp_index].product.to_owned(), qty);
                                // reorder the tampered lists
                                self.apk.fin_prod  = fetch_logs::<FinishedProd>(PathOption::FinProd).unwrap();
                                
                                let mut dy: Vec<Vec<DailyYield>> = Vec::new();
                                {
                                    let p_list = fetch_logs::<Product>(PathOption::Product).unwrap();
                                    for pr in p_list {
                                        let path_str = format!("records/{}dyield",&pr.name);
                                        let path = Path::new(&path_str);
                                        let list = fetch_daily_logs(path).unwrap();
                                        let _ = &dy.push(list);
                                    }
                                }
                                self.tvecs.dy = dy;
                                // finally
                                self.editor.r = false;
                            }
                        }
                    });
                });
            }
            // packing options
            if self.editor.s {
                let string = format!("add {} packed items",self.apk.pkg_prod[self.tvecs.pkg_index].pkg_specify);
                ui.label(RichText::new(&string));
                ui.add_space(5.);
                ui.text_edit_singleline(&mut self.editor.j);
                ui.horizontal(|ui|{
                    ui.with_layout(Layout::right_to_left(), |ui|{
                        ui.add_space(10.);
                        if ui.button(RichText::new("Close")).clicked() {
                            self.editor.s = false;
                        }
                        if ui.button(RichText::new("Add")).clicked() {
                            // choose specific product
                            if let Ok(packs) = self.editor.j.parse::<u32>() {
                                for fp in  self.apk.fin_prod.iter_mut() {
                                    if fp.product == self.apk.pkg_prod[self.tvecs.pkg_index].product {
                                        let qty = self.apk.pkg_prod[self.tvecs.pkg_index].quantity * packs as f32;
                                        // subtract quantity 
                                        fp.quantity -= qty;
                                        break; 
                                    }
                                }

                                // log the change
                                let path = Path::new("records/finprod");
                                let item_log=serde_yaml::to_vec(&self.apk.fin_prod.to_owned()).unwrap();
                                let mut file=fs::File::create(path).expect("cant open file");
                                file.write_all(&item_log).expect("cant write into..");
                                
                                self.apk.pkg_prod[self.tvecs.pkg_index].total += packs;
                                // log the change
                                let path2 = Path::new("records/pkgprod");
                                let item_log2 = serde_yaml::to_vec(&self.apk.pkg_prod.to_owned()).unwrap();
                                let mut file = fs::File::create(path2).expect("cant open file");
                                file.write_all(&item_log2).expect("cant write into..");
                                
                                self.editor.s = false;
                            }
                        }
                    });
                });
            }
            if self.editor.z {
                let string = format!("edit {}'s price",self.apk.pkg_prod[self.tvecs.pkg_index].pkg_specify);
                ui.label(RichText::new(&string));
                ui.add_space(5.);
                ui.text_edit_singleline(&mut self.editor.j);
                ui.horizontal(|ui|{
                    ui.with_layout(Layout::right_to_left(), |ui|{
                        ui.add_space(10.);
                        if ui.button(RichText::new("Close")).clicked() {
                            self.editor.z = false;
                        }
                        if ui.button(RichText::new("Confirm")).clicked() {
                            // choose specific product
                            if let Ok(price) = self.editor.j.parse::<f32>() {
                                self.apk.pkg_prod[self.tvecs.pkg_index].cost = price;

                                // log the change
                                let path2 = Path::new("records/pkgprod");
                                let item_log2 = serde_yaml::to_vec(&self.apk.pkg_prod.to_owned()).unwrap();
                                let mut file = fs::File::create(path2).expect("cant open file");
                                file.write_all(&item_log2).expect("cant write into..");
                                
                                self.editor.z = false;
                            }
                        }
                    });
                });
            }
        });


        CentralPanel::default().frame(frame).show(ctx, |ui|{
            ui.set_style(crate::styles::top_panel_style());
            ui.add_space(10.);

            ui.columns(3, |a|{
                // a[1].visuals_mut().override_text_color = Some(Color32::GRAY);
                // ////////////////////////////////////////////////////////////////
                a[0].label(RichText::new("Products"));

                a[0].separator();
                if self.apk.product.is_empty() {
                    a[0].add_space(30.);
                    a[0].label(RichText::new("oops!! no product yet!!").color(Color32::RED));
                }else{
                    ScrollArea::vertical().id_source("product scroll").max_height(210.)
                    .show(&mut a[0], |ui| {
                        for (i, pr) in self.apk.product.iter().enumerate() {
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.add_space(10.);
                                    ui.label(RichText::new(&pr.name));
                                });
                            });
                        }
                    });
                }
                // ////////////////////////////////////////////////////////////////
                if !self.editor.n {
                    a[1].horizontal(|ui| {
                        ui.label(RichText::new("Recent Productions"));
                        ui.separator();
                        if ui.button(RichText::new("initiate new")).clicked() {
                            // new production logic
                            self.editor.n = true;
                        }
                    });
                    a[1].separator();

                    if self.apk.production.is_empty() {
                        a[1].add_space(30.);
                        a[1].label(RichText::new("oops! nothing here!!").color(Color32::RED));
                    }else{
                        ScrollArea::vertical().id_source("prodscroll").max_height(200.)
                        .show(&mut a[1], |ui|{
                            for (i, prod) in self.tvecs.production.iter().enumerate() {
                                let date = format!(
                                    "{}-{}-{}",
                                    &prod.date.day,
                                    &prod.date.month,
                                    &prod.date.year 
                                );
                                ui.horizontal(|ui| {
                                    ui.with_layout(Layout::left_to_right(), |ui|{
                                        ui.label(RichText::new(&date).color(Color32::DARK_BLUE));
                                        ui.add_space(5.);
                                        ui.label(RichText::new(&prod.product.name));
                                    });
                                    ui.with_layout(Layout::right_to_left(), |ui|{
                                        if ui.button(RichText::new("more info")).clicked() {
                                            self.editor.m = true;
                                            self.tvecs.index = i;
                                        }
                                        ui.add_space(5.);
                                    });
                                });
                            }
                        });
                    }
                }else{
                    // new production
                    a[1].horizontal(|ui|{
                        let back =ui.add(Button::new(RichText::new("◀ back")));
                        if back.clicked() {
                            self.editor.n = false;
                        }
                        ui.label(RichText::new("To Produce ..."));
                    });
                    a[1].separator();

                    a[1].add_space(5.);
                    a[1].horizontal(|ui|{
                        ui.label(RichText::new("Product"));
                        ui.add_space(5.);
                        let choose = ui.button("choose"); ui.add_space(5.);
                        if choose.clicked() {
                            self.editor.p = true; // redundant *******************************
                        }
                        if !self.tvecs.prod_actual.is_empty() {
                            ui.label(RichText::new(&self.tvecs.prod_actual[0].name).color(Color32::DARK_BLUE));
                        }
                    });
                    a[1].label(RichText::new("Materials .."));
                    ScrollArea::vertical().id_source("raw_p_scroll").max_height(130.)
                    .show(&mut a[1], |ui|{
                        let mut ind: usize = 0; 
                        for (i, item) in self.tvecs.actual_item_list.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.label(RichText::new(&item.name));
                                    ui.separator();
                                    let qty = format!("qty: {}kg",item.quantity);
                                    ui.label(RichText::new(&qty));
                                });
                                ui.with_layout(Layout::right_to_left(), |ui|{
                                    ui.add_space(5.);
                                    if ui.button(RichText::new("❎")).clicked() {
                                        ind = i;
                                        self.editor.o = true;
                                    }
                                    ui.separator();
                                });
                            });
                        }
                        if self.editor.o {
                            self.tvecs.actual_item_list.remove(ind);
                            self.editor.o = false;
                        }
                    });
                    a[1].add_space(7.);
                    if !self.tvecs.prod_actual.is_empty() && !self.tvecs.actual_item_list.is_empty(){
                        if a[1].button(RichText::new("DONE")).clicked() {
                            let mut p = Production::new(self.tvecs.prod_actual[0].to_owned());
                            for rm in &self.tvecs.actual_item_list {
                                p.add_rawmat(rm);
                            }
                            let path = Path::new("records/production");
                            p.log(path);
                            self.editor.n = false;
                            // house cleaning
                            self.tvecs.prod_actual = Vec::new();
                            self.tvecs.actual_item_list = Vec::new();
                            // re-fetch the raw_materials list and production list
                            self.apk.raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
                            self.apk.production = fetch_logs::<Production>(PathOption::Production).unwrap();
                            //the sneaky tvecs too
                            let mut list = self.apk.production.to_owned();
                            list.reverse();
                            self.tvecs.production = list;
                        }
                    }

                }
                // ////////////////////////////////////////////////////////////////////////////////////
                if !self.editor.n {
                    a[2].indent("third", |ui| {
                        ui.label(RichText::new("More Info Tab"));
                        // more (if)s logic
                        if self.editor.m {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&self.tvecs.production[self.tvecs.index].product.name));
                                ui.separator();
                                let date = format!(
                                    "{}-{}-{}",
                                    &self.tvecs.production[self.tvecs.index].date.day,
                                    &self.tvecs.production[self.tvecs.index].date.month,
                                    &self.tvecs.production[self.tvecs.index].date.year 
                                );
                                ui.label(RichText::new(&date).color(Color32::DARK_BLUE));
                            });
                            ScrollArea::vertical().id_source("prodmatscroll").max_height(200.)
                            .show(ui, |ui|{
                                for prod_rms in self.tvecs.production[self.tvecs.index].raw_mat.iter() {
                                    ui.horizontal(|ui|{
                                        ui.with_layout(Layout::left_to_right(),|ui|{
                                            ui.add_space(10.);
                                            ui.label(RichText::new(&prod_rms.name));
                                        });
                                        ui.with_layout(Layout::right_to_left(),|ui|{
                                            ui.add_space(20.);
                                            ui.label(RichText::new(&prod_rms.quantity.to_string()));
                                        });
                                    });
                                }
                            });
                        }
                    });
                }else if self.editor.p {
                    // product list
                    a[2].horizontal(|ui|{
                        let back =ui.add(Button::new(RichText::new("◀ back")));
                        if back.clicked() {
                            self.editor.p = false;
                        }
                        ui.separator();
                        ui.label("Search").on_hover_text("type to narrow search");
                    });
                    a[2].add_space(5.);
                    let search = a[2].text_edit_singleline(&mut self.editor.f);
                    a[2].add_space(5.);
                    a[2].separator();

                    if search.changed() {
                        let p: Vec<_> = self.apk.product.to_owned().into_iter().filter(|pr|{
                            pr.name.contains(&self.editor.f)
                        }).collect();
                        self.tvecs.prod = p;
                    }

                    ScrollArea::vertical().id_source("pr_pr_scroll").max_height(180.)
                    .show(&mut a[2], |ui|{
                        for  sp in self.tvecs.prod.iter(){
                            ui.label(&sp.name);
                            if ui.button("pick").clicked(){
                                self.tvecs.prod_actual = Vec::new();
                                self.tvecs.prod_actual.push(sp.to_owned());
                                self.editor.p = false;
                            };
                            ui.separator();
                        }
                    });
                }else if self.editor.q {
                    a[2].horizontal(|ui| {
                        let back =ui.add(Button::new(RichText::new("◀ back")));
                        if back.clicked() {
                            self.editor.q = false;
                        }
                        ui.label(RichText::new("quantity of .."));
                        if !self.tvecs.item_list.is_empty() {
                            let i = self.tvecs.item_list.len()-1;
                            let name = &self.tvecs.item_list[i];
                            ui.label(RichText::new(name));
                        }
                    });
                    a[2].add_space(5.);
                    a[2].text_edit_singleline(&mut self.editor.h);
                    a[2].horizontal(|ui|{
                        ui.add_space(50.);
                        if ui.button(RichText::new("Add")).clicked() {
                            if let Ok(qty) = self.editor.h.parse::<f32>() {
                                // if qty > &self.tvecs.item_list
                                let i = self.tvecs.item_list.len()-1;
                                let name = self.tvecs.item_list[i].to_owned();
                                if let Some((mut rm, avail_qty)) = RawMaterial::sell_rm(name){
                                    if qty <= avail_qty {
                                        rm.quantity = qty;

                                        self.tvecs.actual_item_list.push(rm);
                                        self.editor.q = false
                                    }
                                }


                            }
                        }
                        if ui.button(RichText::new("Close")).clicked() { self.editor.q = false }
                    });

                }else {
                    a[2].horizontal( |ui| {
                        ui.label(RichText::new("material search"));
                        let search = ui.text_edit_singleline(&mut self.editor.d);
                        if search.changed() {
                            let p: Vec<_> = self.apk.raw_mat.to_owned().into_iter().filter(|rm|{
                                rm.name.contains(&self.editor.d)
                            }).collect();
                            dbg!(&p);
                            self.tvecs.rm = p;
                        }
                    });
                    a[2].separator();
        
                    ScrollArea::vertical().id_source("search_prod_scroll").max_height(180.)
                      .show(&mut a[2], |ui|{
                        for rm in self.tvecs.rm.iter() {
                            ui.label(RichText::new(&rm.name));
                            ui.add_space(5.);
                            let tstr = format!("remaining quantity: {}",&rm.quantity);
                            ui.label(RichText::new(tstr));
                            ui.add_space(5.);
                            if ui.button("pick").clicked() {
                                // add to list // pop a window
                                self.tvecs.item_list.push(rm.name.to_owned()); // might changge
                                dbg!(&self.tvecs.item_list);
                                self.editor.q = true;
                            }
                            ui.add_space(5.);
                            ui.separator();
                        }
                    });
                }
                // ///////////////////////////////////////////////////////////////////////////////////
            });
            ui.separator();


            ui.columns(2, |b|{
                // //////////////////////////////////////////////////////////////////
                b[0].label(RichText::new("Daily Records").strong().underline());

                // b[0].separator();
                ScrollArea::vertical().id_source("dyscroll").max_height(200.)
                .show(&mut b[0], |ui|{
                    for record in &self.tvecs.dy{
                        if record.len() > 0 {
                            let i = record.len()-1;
                            let date = format!(
                                "{}-{}-{}",
                                &record[i].date.day,
                                &record[i].date.month,
                                &record[i].date.year 
                            );
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(),|ui|{
                                    ui.label(RichText::new(&date).color(Color32::DARK_BLUE));
                                    ui.separator();
                                    ui.label(RichText::new(&record[i].product.name));
                                });
                                ui.with_layout(Layout::right_to_left(),|ui|{
                                    ui.add_space(8.);
                                    let temp = format!("{} kg",&record[i].quantity.to_string());
                                    ui.label(RichText::new(&temp).color(Color32::BROWN));
                                });
                            });
                            ui.add_space(8.);
                            // ui.separator();
                        }
                    }
                });

                // ////////////////////////////////////////////////////////////////////
                b[1].add_space(10.);
                b[1].label(RichText::new("Raw Materials").underline());
                b[1].add_space(7.);
                if self.apk.raw_mat.is_empty() {
                    b[1].add_space(30.);
                    b[1].label(RichText::new("oops!! No Raw Materials in stock.").color(Color32::RED));
                }else{
                    ScrollArea::vertical().id_source("rawmatcsroll")
                    .show(&mut b[1], |ui| {
                        for item in self.apk.raw_mat.iter() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&item.name));
                                ui.separator();
                            });
                            ui.add_space(5.);
                            ui.horizontal(|ui|{
                                let qty = format!("quantity: {}kg",item.quantity);
                                ui.label(RichText::new(&qty));
                                ui.separator();
                            });
                            ui.add_space(5.);
                            ui.separator();
                        } 
                    });
                }
            });

        });

    }

    pub fn render_sales_win(&mut self, ctx: &CtxRef){
        let frame = crate::styles::top_panel_frame();
        let sp = SidePanel::new(Side::Left, "side_menu").min_width(130.).max_width(130.).frame(frame);
        sp.show(ctx, |ui|{
            
            ui.set_style(crate::styles::top_panel_style());

            ui.add_space(10.);
            let trans = ui.add(Label::new(RichText::new("transactions")
              .strong().color(
                  if self.conf.sale_config.normal_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
              ) ).sense(Sense::click()));
            if trans.clicked(){
                self.conf.sale_config.normal_win = true;
                self.conf.sale_config.debt_win = false;
            }
            ui.add(Separator::default().spacing(10.) );
            let dt = ui.add(Label::new(RichText::new("debts")
              .strong().color(
                if self.conf.sale_config.debt_win {Color32::from_rgb(91,40,195)} else {Color32::BLACK}
              ) ).sense(Sense::click()) );
            if dt.clicked(){
                self.conf.sale_config.normal_win = false;
                self.conf.sale_config.debt_win = true;
            }
            ui.add(Separator::default().spacing(10.) );

        });
        
        if self.conf.sale_config.normal_win {
            CentralPanel::default().frame(frame).show(ctx, |ui|{
                ui.add_space(15.);
                egui::menu::bar(ui, |ui|{

                    ui.set_style(crate::styles::top_panel_style());

                    let rt = RichText::new("buy").color(
                        if self.conf.sale_normal_config.buy_win {Color32::from_rgb(91,40,195)}else { Color32::BLACK }
                    );
                    let rst = RichText::new("sell").color(
                        if self.conf.sale_normal_config.sell_win {Color32::from_rgb(91,40,195)}else { Color32::BLACK }
                    );
                    let ls = ui.button(rst.heading()); 
                    ui.separator();
                    let lb = ui.button(rt.heading());
                    ui.separator();

                    if lb.clicked() {
                        self.conf.sale_normal_config.buy_win = true;
                        self.conf.sale_normal_config.sell_win = false;
                    }
                    if ls.clicked() {
                        self.conf.sale_normal_config.buy_win = false;
                        self.conf.sale_normal_config.sell_win = true;
                    }

                });
                if self.conf.sale_normal_config.buy_win {
                    ui.add_space(20.);
                    self.buy_window(ui);
                }else if self.conf.sale_normal_config.sell_win {
                    ui.add_space(20.);
                    self.sell_window(ui);
                }
                
            });

        }else if self.conf.sale_config.debt_win {
            CentralPanel::default().frame(frame).show(ctx, |ui|{
                ui.set_style(crate::styles::top_panel_style());

                egui::menu::bar(ui, |ui|{
          
                    let rt = RichText::new("Debtors").color(if self.conf.sale_debt_config.debtors{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let rst = RichText::new("Creditors").color(if self.conf.sale_debt_config.creditors{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                    let lb = ui.button(rt.heading());
                    ui.separator();
                    let ls = ui.button(rst.heading()); 
                    ui.separator();

                    if lb.clicked() {
                        self.tvecs.index = 0;
                        self.conf.sale_debt_config.debtors = true;
                        self.conf.sale_debt_config.creditors = false;
                    }
                    if ls.clicked() {
                        self.tvecs.index = 0;
                        self.conf.sale_debt_config.creditors = true;
                        self.conf.sale_debt_config.debtors = false;
                    }
                });
                if self.conf.sale_debt_config.debtors {
                    ui.add_space(20.);
                    self.debtors_haven(ui);
                }else if self.conf.sale_debt_config.creditors {
                    ui.add_space(20.);
                    self.creditors_haven(ui);
                }
            });
        }
    }

    pub fn debtors_haven(&mut self, ui: &mut Ui) {

        ui.columns(3,|col|{
            // set the temp bools
            col[0].label(RichText::new("Unaowadai"));
            col[0].separator();

            ScrollArea::vertical().show(&mut col[0], |ui|{
                for (i, p) in self.apk.debtors.iter().enumerate() {
                    ui.label(&p.person.name);
                    ui.label(RichText::new(&p.person.tel).small() );
                    ui.label(RichText::new(format!("Ksh. {}",&p.total_amount)).color(Color32::RED) );
                    ui.horizontal(|ui|{
                        ui.with_layout(Layout::right_to_left(), |ui|{
                            ui.add_space(7.);
                            if ui.button("more ..").clicked() { 
                                self.editor.g = i.to_string();
                                self.editor.s = true; 
                                self.editor.r = false 
                            }
                        });
                    });
                    
                    ui.separator();
                }
            });

            if self.editor.s {
                col[1].add_space(20.);

                let mut success_validate = false;

                if let Ok(index) = self.editor.g.parse::<usize>(){
                    if self.apk.debtors.get(index).is_some() {
                        let mut de =  &mut self.apk.debtors[index];
    
                        col[1].label(RichText::new(&de.person.name).underline());
                        col[1].add_space(10.);
                        col[1].horizontal(|ui|{
                            ui.label("amount to pay: ");
                            ui.add_space(10.);
                            ui.label(RichText::new(&de.total_amount.to_string()).color(Color32::RED));
                        });
                        col[1].add_space(15.);
                        col[1].text_edit_multiline(&mut self.editor.f);
                        col[1].add_space(10.);
                        if col[1].button("settle ..").clicked(){
                            if let Ok(bill) = self.editor.f.parse::<u32>() {
                                de.settle_debt(bill, &mut self.apk.money_in); 
                                success_validate = true;
                            }
                        }
    
                        col[1].add(Separator::default().spacing(10.));
                        col[1].label("associated transactions");
                        col[1].add(Separator::default().spacing(10.));
                        ScrollArea::vertical().id_source("scroll2").show(&mut col[1], |ui|{
                            for trans in &self.apk.money_in {
                                if trans.balance.is_some() && trans.person == de.person{
                                    // ******* if i ever add another product.. this will break ******* //
                                    ui.label(&trans.time);
                                    ui.label(RichText::new(format!("quantity: {}",&trans.items[0].quantity)));
                                    ui.label(RichText::new(format!("total cost: {}",&trans.total_cost)));
                                    ui.label(RichText::new(format!("unpaid balance: {}",&trans.balance.unwrap())));
                                    ui.separator();
                                }
                            }
                        });
                        if success_validate {
                            // log creditors list
                            let path_a = Path::new("records/debtors");
                            let item_log = self.apk.debtors.to_owned().into_iter().filter(|per|per.total_amount > 0).collect::<Vec<Debtor>>();
                            let item_log=serde_yaml::to_vec(&item_log).unwrap();
                            let mut file=fs::File::create(path_a).expect("cant open file");
                            file.write_all(&item_log).expect("cant write into..");
    
                            self.apk.debtors = fetch_logs::<Debtor>(PathOption::Debtors).unwrap();
                        }
                    }
                }
            }
            
            col[2].label(RichText::new("search corner"));
            col[2].separator();
            let search = col[2].text_edit_singleline(&mut self.editor.a);
            
            if search.changed() {
                let p: Vec<_> = self.apk.debtors.to_owned().into_iter().filter(|per|{
                    per.person.name.contains(&self.editor.a)
                }).collect();
                self.tvecs.debt = p;
            }
            ScrollArea::vertical().id_source("scroll3").show(&mut col[2], |ui|{
                for p in self.tvecs.debt.iter() {
                    ui.label(&p.person.name);
                    ui.label(RichText::new(&p.person.tel).small() );
                    ui.label(RichText::new(format!("Ksh. {}",&p.total_amount)).color(Color32::RED) );
                    if ui.button("more ..").clicked() { 

                        for (i,debtee) in self.apk.debtors.iter().enumerate() {
                            if debtee.person == p.person {
                                self.editor.g = i.to_string();
                                self.editor.s = true; 
                                self.editor.r = false;
                                break;
                            }
                        }
                    }
                    ui.separator();
                }
            });
        });
    }
    
    pub fn creditors_haven(&mut self, ui: &mut Ui) {
        ui.columns(3,|col|{
            // set the temp bools
            col[0].label(RichText::new("Wanaokudai"));
            col[0].separator();

            ScrollArea::vertical().show(&mut col[0], |ui|{
                for (i, p) in self.apk.creditors.iter().enumerate() {
                    ui.label(&p.person.name);
                    ui.label(RichText::new(&p.person.tel).small() );
                    ui.label(RichText::new(format!("Ksh. {}",&p.total_amount)).color(Color32::RED) );
                    ui.horizontal(|ui|{
                        ui.with_layout(Layout::right_to_left(), |ui|{
                            ui.add_space(7.);
                            if ui.button("more ..").clicked() { 
                                self.editor.g = i.to_string();
                                self.editor.s = true; 
                                self.editor.r = false 
                            }
                        });
                    });
                    
                    ui.separator();
                }
            });

            if self.editor.s {
                col[1].add_space(20.);

                let mut success_validate = false;

                if let Ok(index) = self.editor.g.parse::<usize>(){

                    if self.apk.creditors.get(index).is_some() {

                        let mut cr =  &mut self.apk.creditors[index];
    
                        col[1].label(RichText::new(&cr.person.name).underline());
                        col[1].add_space(10.);
                        col[1].horizontal(|ui|{
                            ui.label("amount to pay: ");
                            ui.add_space(10.);
                            ui.label(RichText::new(&cr.total_amount.to_string()).color(Color32::RED));
                        });
                        col[1].add_space(15.);
                        col[1].text_edit_multiline(&mut self.editor.f);
                        col[1].add_space(10.);
                        if col[1].button("settle here ..").clicked(){
                            if let Ok(bill) = self.editor.f.parse::<u32>() {
                                cr.settle_debt(bill, &mut self.apk.money_out); 
                                success_validate = true;
                            }
                        }
                        
    
                        col[1].add(Separator::default().spacing(10.));
                        col[1].label("associated transactions");
                        col[1].add(Separator::default().spacing(10.));
                        ScrollArea::vertical().id_source("scroll2").show(&mut col[1], |ui|{
                            for trans in &self.apk.money_out {
                                if trans.balance.is_some() && trans.person == cr.person{
                                    ui.label(&trans.time);
                                    ui.label(RichText::new(format!("total cost: {}",&trans.total_cost)));
                                    ui.label(RichText::new(format!("unpaid balance: {}",&trans.balance.unwrap())));
                                    ui.separator();
                                }
                            }
                        });
    
                        if success_validate {
                            // log creditors list
                            let path_a = Path::new("records/creditors");
                            let item_log=self.apk.creditors.to_owned().into_iter().filter(|per| per.total_amount > 0 ).collect::<Vec<Creditor>>();
                            let item_log=serde_yaml::to_vec(&item_log).unwrap();
                            let mut file=fs::File::create(path_a).expect("cant create file");
                            file.write_all(&item_log).expect("cant write into..");
    
                            self.apk.creditors = fetch_logs::<Creditor>(PathOption::Creditor).unwrap();
                        }
                    }
                }
            }
            
            col[2].label(RichText::new("search corner"));
            col[2].separator();
            let search = col[2].text_edit_singleline(&mut self.editor.a);
            
            if search.changed() {
                let p: Vec<_> = self.apk.creditors.to_owned().into_iter().filter(|per|{
                    per.person.name.contains(&self.editor.a)
                }).collect();
                self.tvecs.credit = p;
            }
            ScrollArea::vertical().id_source("scroll3").show(&mut col[2], |ui|{
                for p in self.tvecs.credit.iter() {
                    ui.label(&p.person.name);
                    ui.label(RichText::new(&p.person.tel).small() );
                    ui.label(RichText::new(format!("Ksh. {}",&p.total_amount)).color(Color32::RED) );
                    if ui.button("more ..").clicked() { 

                        for (i,shylock) in self.apk.creditors.iter().enumerate() {
                            if shylock.person == p.person {
                                self.editor.g = i.to_string();
                                self.editor.s = true; 
                                self.editor.r = false;
                                break;
                            }
                        }
                    }
                    ui.separator();
                }
            });
        });
    }

    pub fn render_logs_win(&mut self, ctx: &CtxRef) {
        let frame = top_panel_frame();
        CentralPanel::default().frame(frame)
        .show(ctx, |ui|{
            ui.set_style(crate::styles::top_panel_style());

            ui.columns(3, |col| {
                col[0].vertical_centered(|ui|{
                    ui.label(RichText::new("all 'Sells' ").underline());
                }); 
                col[0].add_space(8.);
                ScrollArea::vertical().id_source("l_scroll")
                .show(&mut col[0], |ui|{
                    for tr in self.tvecs.in_trans.iter() {
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new(&tr.person.name));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.label(RichText::new(&tr.time).color(Color32::DARK_BLUE));
                            });
                        });
                        ui.add_space(5.);
                        ui.label(RichText::new("items"));
                        ui.add_space(5.);
                        
                        for item in tr.items.iter() {
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.add_space(5.);
                                    ui.label(RichText::new(&item.pkg_specify).color(Color32::DARK_GRAY));
                                    let fstr = format!("at Ksh.{}",item.cost );
                                    ui.label(RichText::new(&fstr).color(Color32::DARK_GRAY))
                                });
                                ui.with_layout(Layout::right_to_left(), |ui|{
                                    ui.add_space(5.);
                                    let fstr = format!("{} packs",item.total);
                                    ui.label(RichText::new(&fstr).color(Color32::DARK_GRAY));
                                });
                            });
                        }
                    
                        ui.add_space(5.);
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.add_space(5.);
                                let fstr = format!("Total Amt: {}",&tr.total_cost);
                                ui.label(RichText::new(&fstr));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.add_space(5.);
                                if let Some(bal) = tr.balance {
                                    let fstr = format!("Bal: .Ksh{}",bal);
                                    ui.label(RichText::new(&fstr));
                                }else {
                                    let fstr = format!("Bal: None");
                                    ui.label(RichText::new(&fstr));
                                }
                            });
                        });

                    ui.separator();                        
                    }
    
                });
                col[2].vertical_centered(|ui|{
                    ui.label(RichText::new("all 'Buys' ").underline());
                });
                col[2].add_space(8.);
                ScrollArea::vertical().id_source("r_scroll")
                .show(&mut col[2], |ui|{
                    for tr in self.tvecs.out_trans.iter() {
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.label(RichText::new(&tr.person.name));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.label(RichText::new(&tr.time).color(Color32::DARK_BLUE));
                            });
                        });
                        ui.add_space(5.);
                        ui.label(RichText::new("items"));
                        ui.add_space(5.);
                        
                        for item in tr.items.iter() {
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.add_space(5.);
                                    ui.label(RichText::new(&item.name).color(Color32::DARK_GRAY));
                                    let fstr = format!("->{} Kg(s)",item.quantity );
                                    ui.label(RichText::new(&fstr).color(Color32::DARK_GRAY))
                                });
                                ui.with_layout(Layout::right_to_left(), |ui|{
                                    ui.add_space(5.);
                                    let fstr = format!("cost: Ksh.{}",item.quantity*item.price_per.unwrap());
                                    ui.label(RichText::new(&fstr).color(Color32::DARK_GRAY));
                                });
                            });
                        }
                    
                        ui.add_space(5.);
                        ui.horizontal(|ui|{
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                ui.add_space(5.);
                                let fstr = format!("Total Amt: {}",&tr.total_cost);
                                ui.label(RichText::new(&fstr));
                            });
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                ui.add_space(5.);
                                if let Some(bal) = tr.balance {
                                    let fstr = format!("Bal: .Ksh{}",bal);
                                    ui.label(RichText::new(&fstr));
                                }else {
                                    let fstr = format!("Bal: None");
                                    ui.label(RichText::new(&fstr));
                                }
                            });
                        });

                    ui.separator();                        
                    }
    
                });
            });
        });
    }

    pub fn render_staff_win(&mut self, ctx: &CtxRef) {
        let frame = top_panel_frame();
        CentralPanel::default().frame(frame).show(ctx, |ui|{
            ui.set_style(crate::styles::top_panel_style());

            ui.columns(2,|col|{
                col[0].vertical_centered(|ui|{
                    ui.add_space(10.);
                    ui.label(RichText::new("Staff List").underline());
                    ui.add_space(10.);
                    ScrollArea::vertical().id_source("staff_scroll").show(ui, |ui|{
                        for (i, per) in self.apk.staff.iter().enumerate() {
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.add_space(15.);
                                    ui.label(RichText::new(&per.name));
                                });
                                ui.with_layout(Layout::right_to_left(), |ui|{
                                    ui.add_space(15.);
                                    let x = match per.active {
                                        true => RichText::new("ACTIVE").color(Color32::GREEN),
                                        false => RichText::new("INACTIVE").color(Color32::LIGHT_RED),
                                    };
                                    ui.label(x);
                                    ui.separator();
                                    let y = match per.sex {
                                        edsa_pos::pipeline::people::Sex::Male => RichText::new("Male"),
                                        edsa_pos::pipeline::people::Sex::Female => RichText::new("Female"),
                                    };
                                    ui.label(y);
                                });
                            });
                            ui.horizontal(|ui|{
                                ui.with_layout(Layout::left_to_right(), |ui|{
                                    ui.add_space(15.);
                                    ui.label(RichText::new(format!("☎ {}",&per.tel)).color(Color32::DARK_GRAY));
                                });
                            });
                            ui.add_space(8.);
                            ui.add(Separator::default());
                        }
                    });
                });

                col[1].indent("reg", |ui|{
                    ui.add_space(15.);
                    ui.vertical_centered(|ui|{
                        ui.label(RichText::new("Registration").underline());
                    });
                    ui.horizontal(|ui|{
                        ui.add_space(20.);
                        ui.label(RichText::new("Name"));
                        ui.add_space(20.);
                        let _text_input = ui.text_edit_singleline( &mut self.editor.c );
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
                        ui.checkbox(&mut self.editor.y, RichText::new("active").monospace());
                    });
                    ui.add_space(20.);
        
                    ui.horizontal(|ui|{
                        ui.add_space(20.);
                        ui.label("Sex: ");
                        ui.selectable_value(&mut self.tvecs.sex , Sex::Male, RichText::new("Male") );
                        ui.separator();
                        ui.selectable_value(&mut self.tvecs.sex, Sex::Female, RichText::new("Female"));
                    });
                    ui.add_space(20.);
        
                    ui.horizontal(|ui|{
                        ui.with_layout(Layout::right_to_left(), |ui|{
                            
                            ui.add_space(20.);
                            
                            if ui.button(RichText::new("add").text_style(TextStyle::Body)).clicked(){
                                if self.editor.c.len()>1 && self.editor.b.len()>1 {
                                    let name = self.editor.c.clone();
                                    let tel = self.editor.b.clone();
                                    let active = self.editor.y;
                                    let sex = self.tvecs.sex;
            
                                    let p = Employee::new(name, sex, active, tel);
            
                                    self.apk.staff.push(p.clone());
                                    let path = Path::new("records/employees");
                                    p.log(path);
                                    self.editor.c = String::default();
                                    self.editor.b = String::default();
                                } 
                            }
                        })
                    });
                    ui.add(Separator::default());
        
                });
            });
        });
    }

    pub fn configure_fonts(&self, ctx: &CtxRef){
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "broadway".to_owned(),
            egui::FontData::from_static(include_bytes!("/home/klan/edsa/edsa_feeds/fonts/BlackgroundsRegular-1GEYj.ttf")),
        );
        font_def.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Monospace, 25.),
        );
        font_def.family_and_size.insert(
            TextStyle::Heading, 
            (FontFamily::Monospace, 30.),
        );
        font_def.fonts_for_family.get_mut(&FontFamily::Monospace)
         .unwrap()
         .insert(0, "broadway".to_owned());

        ctx.set_fonts(font_def);
    }
}

