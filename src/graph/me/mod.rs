//! This mod will  contain different method and struck  used to interacting with
//! ME API
//!
//! The /me node is a special endpoint that translates to the object ID of
//! the person or Page whose access token is currently being used to make the
//! API calls.
//!
//!  If you had a User access token, you could
//! retrieve a User's name and ID by using: The data in the response
//! will depend on the "Fields" parameters  you pass along the get request
//! exmaple fields=id,name,email,picture......

use crate::graph::accounts::AccountsAPI;
use crate::prelude::errors::ClientErr;
use crate::prelude::{Accounts, HttpConnection};
use serde::{Deserialize, Serialize};

/// This struct contain different data gotten as a response  when a user sign in
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Me {
    name: String,
    id: String,
    last_name: String,
    first_name: String,
    picture: PictureData,
    email: Option<String>,
}

impl Me {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn picture(&self) -> &PictureData {
        &self.picture
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PictureData {
    pub data: FacebookPictureUserPicture,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct FacebookPictureUserPicture {
    url: String,
}

impl FacebookPictureUserPicture {
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeApi {
    url: String,
}

impl MeApi {
    pub fn new(graph_base: String) -> MeApi {
        MeApi {
            url: graph_base.replace("NODE", "me"),
        }
    }

    /// This method will get the list of Facebook Pages that a person owns or
    /// is able to perform tasks on.
    ///
    /// To check the possible data that is possible to be in the response which
    /// varies with pages depending on the page privacy
    ///  
    /// [facebook accounts docs](https://developers.facebook.com/docs/graph-api/reference/user/accounts/)
    pub async fn pages_by_me(self) -> Result<Accounts, ClientErr> {
        Ok(AccountsAPI::new(self.url).get().await?)
    }

    pub async fn pages_by_user_id(self, user_id: String) -> Result<Accounts, ClientErr> {
        Ok(AccountsAPI::new(self.url.replace("me", &user_id))
            .get()
            .await?)
    }

    /// The /me node is a special endpoint that translates to the object ID of
    /// the person or Page whose access token is currently being used
    /// to make the API calls. If you had a User access token, you could
    pub async fn user(&self) -> Result<Me, ClientErr> {
        let fields =
            "&fields=id,name,picture, email,first_name,last_name,about,birthday,gender,link";
        let base_ur = self.url.replace("EDGE", "");
        let url = base_ur + fields;

        let resp = HttpConnection::get::<Me>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// The /me node is a special endpoint that translates to the object ID of
    /// the person or Page whose access token is currently being used
    /// to make the API calls. If you had a User access token, you could
    pub async fn user_by_id(&self, user_id: String) -> Result<Me, ClientErr> {
        let base_url = self.url.replace("me", &user_id);

        let fields =
            "&fields=id,name,picture, email,first_name,last_name,about,birthday,gender,link";
        let base_url = base_url.replace("EDGE", "");
        let url = base_url + fields;

        let resp = HttpConnection::get::<Me>(url, "".to_string()).await?;
        Ok(resp)
    }
}
