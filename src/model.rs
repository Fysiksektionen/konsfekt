use crate::database::model::ProductRow;


#[derive(serde::Deserialize)]
pub struct ProductParams {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub price: Option<f32>,
    pub description: Option<String>,
    pub stock: Option<i32>,
    pub flags: Option<ProductFlags>
}

#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub stock: Option<i32>,
    pub flags: ProductFlags,
}

impl Product {
    pub fn from_request(params: ProductParams) -> Result<Product, ()> {
        Ok(Product { 
            id: 0,
            name: params.name.ok_or(())?,
            price: params.price.ok_or(())?, 
            description: params.description.unwrap_or("".to_string()),
            stock: None,
            flags: match params.flags {
                Some(flags) => flags,
                None => ProductFlags::default(),
            },
        })
    }

    pub fn from_row(row: ProductRow) -> Result<Product, ()> {
        Ok(Product { 
            id: row.id,
            name: row.name,
            price: row.price,
            description: row.description,
            stock: row.stock,
            flags: row.flags.0
        })
    }

    pub fn update(&mut self, params: ProductParams) -> Result<(), ()> {
        if let Some(name) = params.name { self.name = name };
        if let Some(price) = params.price { self.price = price };
        if let Some(description) = params.description { self.description = description };
        if let Some(stock) = params.stock { self.stock = Some(stock) };

        if let Some(flags) = params.flags {
            self.flags = flags;
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
            flags: sqlx::types::Json(self.flags)
        }
    }

}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
