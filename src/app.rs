use eframe::App;
use uuid::Uuid;
use core::slice;
use std::{net::{SocketAddr, SocketAddrV4, Ipv4Addr, IpAddr, Ipv6Addr}, collections::HashMap, ops::Div};

#[derive(Clone)]
pub struct SubnetCalculatorApp { 
    label: String,
    window_store: HashMap<Uuid,SubnetWindowStore>
}

impl Default for SubnetCalculatorApp { // Required for eframe/egui
    fn default() -> Self {
        Self { label: "Subnet Calculator".to_owned(), window_store: HashMap::new()}
    }
}

impl SubnetCalculatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn new_calculation_window(&mut self) {
        let window_id = uuid::Uuid::new_v4();
        let store = SubnetWindowStore{
            ip_addr: IpAddr::V4(Ipv4Addr::new(0,0,0,0)),
            mask: 32,  
            addr_slice_store: AddrSliceStore::default(),
            mask_slice_store: AddrSliceStore::default(),
            wildcard_slice_store: AddrSliceStore::default(),
            is_window_open: true
        };
        
        self.window_store.insert(window_id, store);
    }

    pub fn window_backend(&mut self, ctx: &egui::Context) {
        let mut delete_window_data: Vec<Uuid> = vec![];
        
        for (window_id, window_contents) in &mut self.window_store  {

            if window_contents.is_window_open == false {
                delete_window_data.push(window_id.clone());
                continue;
            }

            egui::Window::new(window_id.to_string().as_str())
                .vscroll(true)
                .collapsible(true)
                .open(&mut window_contents.is_window_open)
                .show(ctx, |ui| {
                // Core window logic

                // Choose IPv4 or IPv6; Destructive
                ui.horizontal_top(|ui| {
                    ui.label("IP Version: ");

                    if window_contents.ip_addr.is_ipv4() {
                        let current_v4 = window_contents.ip_addr.clone();
                        ui.selectable_value(&mut window_contents.ip_addr, current_v4, "IPv4");
                    } else {
                        ui.selectable_value(&mut window_contents.ip_addr, IpAddr::V4(Ipv4Addr::new(0,0,0,0)), "IPv4");
                    }

                    if window_contents.ip_addr.is_ipv6() {
                        let current_v6 = window_contents.ip_addr.clone();
                        ui.selectable_value(&mut window_contents.ip_addr, current_v6, "IPv6");
                    } else {
                        ui.selectable_value(&mut window_contents.ip_addr, IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)), "IPv6");
                    }

                    ui.end_row();
                });

                if window_contents.ip_addr.is_ipv4() { 
                    // IPv4

                    ui.horizontal_top(|ui| {
                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.addr_slice_store.slice_a));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.addr_slice_store.slice_b));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.addr_slice_store.slice_c));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.addr_slice_store.slice_d));
                        ui.add_sized([10.0, 10.0], egui::Label::new("/"));

                        let mut mask = String::from(window_contents.mask.to_string());
                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut mask));
                        let mask_i8 = mask.parse::<i8>();
                        if mask_i8.is_ok() {
                            let unwrapped_mask = mask_i8.unwrap();
                            if unwrapped_mask <= 32 {
                                window_contents.mask = unwrapped_mask;
                            }
                        }

                    });
                    ui.end_row();

                    validate_all_slice_contents(IpVersion::IPv4, &mut window_contents.addr_slice_store);

                    // Mask
                    ui.horizontal_top(|ui| {
                        ui.label("Mask:");

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.mask_slice_store.slice_a));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.mask_slice_store.slice_b));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.mask_slice_store.slice_c));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.mask_slice_store.slice_d));
                    });

                    // Wildcard
                    ui.horizontal_top(|ui| {
                        ui.label("Wildcard:");

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.wildcard_slice_store.slice_a));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.wildcard_slice_store.slice_b));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.wildcard_slice_store.slice_c));
                        ui.add_sized([10.0, 10.0], egui::Label::new("."));

                        ui.add_sized([10.0, 10.0], egui::TextEdit::singleline(&mut window_contents.wildcard_slice_store.slice_d));
                    });


                    // Bits view
                    ui.horizontal_top(|ui| {
                        ui.label("Bits analysis: ");
                    });
                    
                } else if window_contents.ip_addr.is_ipv6() {
                    // IPv6
                    println!("IPv6 Addr: {}", window_contents.ip_addr.to_string());
                }

            });
            
            mask_calculator(IpVersion::IPv4, &mut window_contents.mask, &mut window_contents.mask_slice_store, &mut window_contents.wildcard_slice_store);
            update_addressing(window_contents);
        }

        // Delete closed window data
        for id in delete_window_data {
            self.window_store.remove(&id);
        }

    }

}

impl App for SubnetCalculatorApp { // Required for eframe/egui
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        
        egui::SidePanel::left("side_panel").show(ctx, |ui| {

            if ui.button("New calculation").clicked() {
                self.new_calculation_window();
            }

        });

        self.window_backend(ctx);
    }
}

#[derive(Clone)]
pub struct SubnetWindowStore {
    ip_addr: IpAddr,
    mask: i8,
    addr_slice_store: AddrSliceStore,
    mask_slice_store: AddrSliceStore,
    wildcard_slice_store: AddrSliceStore,
    is_window_open: bool
}

#[derive(Clone)]
pub struct AddrSliceStore {
    slice_a: String,
    slice_b: String,
    slice_c: String,
    slice_d: String,
    slice_e: String, // IPv6 only
    slice_f: String, // IPv6 only
    slice_g: String, // IPv6 only
    slice_h: String  // IPv6 only
}

impl Default for AddrSliceStore {
    fn default() -> Self {
        Self { slice_a: String::from(""), slice_b: String::from(""), slice_c: String::from(""), slice_d: String::from(""), slice_e: String::from(""), slice_f: String::from(""), slice_g: String::from(""), slice_h: String::from("") }
    }
}

fn validate_all_slice_contents(ip_ver: IpVersion, slice_store: &mut AddrSliceStore) { // Too lazy to write an iterator for a for loop, and probably would have more lines to do so

    validate_slice_contents(ip_ver, &mut slice_store.slice_a);
    validate_slice_contents(ip_ver, &mut slice_store.slice_b);
    validate_slice_contents(ip_ver, &mut slice_store.slice_c);
    validate_slice_contents(ip_ver, &mut slice_store.slice_d);
    if ip_ver == IpVersion::IPv6 {
        validate_slice_contents(ip_ver, &mut slice_store.slice_e);
        validate_slice_contents(ip_ver, &mut slice_store.slice_f);
        validate_slice_contents(ip_ver, &mut slice_store.slice_g);
        validate_slice_contents(ip_ver, &mut slice_store.slice_h);
    }


}

fn validate_slice_contents(ip_ver: IpVersion, slice: &mut String) {
    slice.trim();

    if ip_ver == IpVersion::IPv4 {
        
        // Resize to 3 chars (max octet size decimal)
        if slice.len() > 3 {
            slice.drain(3..);
        }

        // Drain invalid characters
        let mut del_pos: Vec<usize> = vec![]; // Position to delete
        for i in 0..slice.len() {
            if slice.chars().nth(i).unwrap().to_digit(10).is_none() { // Char isn't a valid digit
                del_pos.push(i.clone());
            } else if i == 0 
                && slice.len() > 2 
                && slice.chars().nth(0).unwrap().to_digit(10).unwrap() > 2 {
                    del_pos.push(i.clone());
            } else if i > 0 
                && slice.chars().nth(0).unwrap().to_digit(10).unwrap() == 2
                && slice.chars().nth(i).unwrap().to_digit(10).unwrap() > 5 {
                    del_pos.push(i.clone());
            }
        }

        for idx in del_pos {
            slice.drain(idx..idx+1);
        }


    } else if ip_ver == IpVersion::IPv6 {

        slice.to_ascii_lowercase();

        // Resize string to max hextet size
        if slice.len() > 4 {
            slice.drain(4..);
            
        }

        // Drain invalid characters
        let mut del_pos: Vec<usize> = vec![]; // Position to delete
        for i in 0..slice.len() {
            if slice.chars().nth(i).unwrap().to_digit(16).is_none() { // Char isn't a valid digit
                del_pos.push(i.clone());
            }
        }

        for idx in del_pos {
            slice.drain(idx..idx+1);
        }

    }

}

#[derive(PartialEq, Copy, Clone)]
pub enum IpVersion {
    IPv4,
    IPv6
}


pub fn update_addressing_non_mut(store: SubnetWindowStore) -> SubnetWindowStore {
    let mut store2 = store.clone();
    update_addressing(&mut store2);
    store2
}

pub fn update_addressing(store: &mut SubnetWindowStore) {
    
    if store.ip_addr.is_ipv4() {
        store.ip_addr = IpAddr::V4(Ipv4Addr::new(
            store.addr_slice_store.slice_a.parse().unwrap_or(0),
            store.addr_slice_store.slice_b.parse().unwrap_or(0),
            store.addr_slice_store.slice_c.parse().unwrap_or(0), 
            store.addr_slice_store.slice_d.parse().unwrap_or(0)
        ));
    } else if store.ip_addr.is_ipv6() {
        store.ip_addr = IpAddr::V6(Ipv6Addr::new(
            store.addr_slice_store.slice_a.parse().unwrap_or(0),
            store.addr_slice_store.slice_b.parse().unwrap_or(0),
            store.addr_slice_store.slice_c.parse().unwrap_or(0), 
            store.addr_slice_store.slice_d.parse().unwrap_or(0),
            store.addr_slice_store.slice_e.parse().unwrap_or(0),
            store.addr_slice_store.slice_f.parse().unwrap_or(0),
            store.addr_slice_store.slice_g.parse().unwrap_or(0),
            store.addr_slice_store.slice_h.parse().unwrap_or(0)
        ));
    }

}

pub fn mask_calculator(ip_ver: IpVersion, cidr: &mut i8, mask: &mut AddrSliceStore, wildcard: &mut AddrSliceStore) {
    let max: i8 = if ip_ver == IpVersion::IPv4 { 31 } else { 127 };
    let slice_size: i8 = if ip_ver == IpVersion::IPv4 { 8 } else { 16 };

    let cidr_coverage_slices: i8 = *cidr / slice_size;
    let cidr_slice_spill: i8 = *cidr % slice_size; // bits spilling over to the next slice that count as network

    let max_slices = ((max+1) / slice_size);
    for i in 0..max_slices {
        mask_slice_calc(ip_ver, i, cidr_coverage_slices, cidr_slice_spill, mask, wildcard);
    }


}

pub fn mask_slice_calc(ip_ver: IpVersion, slice_id: i8, cidr_coverage_slices: i8, spill: i8, mask: &mut AddrSliceStore, wildcard: &mut AddrSliceStore) {

    if cidr_coverage_slices > slice_id { // Works
        update_slice_from_slice_id(slice_id, String::from("255"), mask);
        update_slice_from_slice_id(slice_id, String::from("0"), wildcard);
    } else if spill == 0 || cidr_coverage_slices+1 != slice_id {
        update_slice_from_slice_id(slice_id, String::from("0"), mask);
        update_slice_from_slice_id(slice_id, String::from("255"), wildcard);
    } 

    // TODO: PARTIAL
    if spill != 0 && cidr_coverage_slices == slice_id {

        if ip_ver == IpVersion::IPv4 {
            let mut octet: u8 = 0b0000_0000;

            match spill {
                8 => octet = 0b1111_1111,
                7 => octet = 0b1111_1110,
                6 => octet = 0b1111_1100,
                5 => octet = 0b1111_1000,
                4 => octet = 0b1111_0000,
                3 => octet = 0b1110_0000,
                2 => octet = 0b1100_0000,
                1 => octet = 0b1000_0000,
                _ => octet = octet
            }

            update_slice_from_slice_id(slice_id, octet.to_string(), mask);
            update_slice_from_slice_id(slice_id, (!octet).to_string(), wildcard);
            
        }

    } 

}

pub fn update_slice_from_slice_id(id: i8, slice_value: String, slice_store: &mut AddrSliceStore) {

    match id {
        0 => slice_store.slice_a = slice_value,
        1 => slice_store.slice_b = slice_value,
        2 => slice_store.slice_c = slice_value,
        3 => slice_store.slice_d = slice_value,
        4 => slice_store.slice_e = slice_value,
        5 => slice_store.slice_f = slice_value,
        6 => slice_store.slice_g = slice_value,
        7 => slice_store.slice_h = slice_value,
        _ => println!("Invalid slice id: {} when updating slice from slice id. ", id)
    }


}