use crate::database::model::ProductRow;


#[derive(serde::Deserialize)]
pub struct ProductParams {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub price: Option<f32>,
    pub description: Option<String>,
    pub flags: Option<String>
}

pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub stock: Option<i32>,
    pub flags: ProductFlags,
}

impl Product {
    pub fn from_params(params: ProductParams) -> Result<Product, ()> {
        let flags = params.flags.ok_or(())?;
        Ok(Product { 
            id: 0,
            name: params.name.ok_or(())?,
            price: params.price.ok_or(())?, 
            description: params.description.unwrap_or("".to_string()),
            stock: None,
            flags: ProductFlags::from_str(&flags)?,
        })
    }

    pub fn update(&mut self, params: ProductParams) -> Result<(), ()> {
        if let Some(name) = params.name { self.name = name };
        if let Some(price) = params.price { self.price = price };
        if let Some(description) = params.description { self.description = description };

        if let Some(flags) = params.flags {
            self.flags = ProductFlags::from_str(&flags)?;
        };

        Ok(())
    }

    pub fn into_row(self) -> ProductRow {
        ProductRow {
            id: self.id,
            name: self.name,
            price: self.price,
            description: self.description,
            stock: self.stock,
            flags: self.flags.to_string()
        }
    }

}

impl ProductParams {
    pub fn get_flags(&self) -> Result<ProductFlags, ()> {
        let Some(flags) = self.flags.clone() else { return Err(()) };
        match serde_json::from_str(&flags) {
            Ok(f) => Ok(f),
            Err(_) => Err(())
        }
    }
}

impl ProductRow {

    pub fn get_flags(&self) -> ProductFlags {
        serde_json::from_str(&self.flags).unwrap()
    }

    pub fn set_flags(&mut self, flags: &ProductFlags) {
        self.flags = flags.to_string();
    }

    pub fn update(&mut self, params: ProductParams) {
        match params.name { Some(name) => self.name = name, _ => () }
        match params.price { Some(price) => self.price = price, _ => () }
        match params.description { Some(description) => self.description = description, _ => () }
        match params.flags { Some(flags) => self.flags = flags, _ => () }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProductFlags {
    pub modifiable: bool, // is only modifiable by admin
    pub new_product: bool, // Example
}

impl ProductFlags {
    pub fn default() -> ProductFlags {
        ProductFlags { 
            modifiable: true,
            new_product: false,
        }
    }

    pub fn from_str(string: &str) -> Result<ProductFlags, ()> {
        serde_json::from_str::<ProductFlags>(string).map_err(|_| ())
    }

    pub fn validate_string(string: &str) -> bool {
        match serde_json::from_str::<ProductFlags>(string) {
            Ok(_) => true,
            Err(_) => false,
        }
    } 

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}