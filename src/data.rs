use sled::Db;
use serenity::model::id::UserId;

type Error = Box<dyn std::error::Error>;

pub struct ScoreDb {
    db: Db,
}

impl ScoreDb {
    pub fn open(path: &str) -> Result<Self, Error> {
        Ok(Self { db: sled::open(path)? })
    }

    pub fn get(&self, user_id: UserId) -> i128 {
        self.db
            .get(user_id.get().to_be_bytes())
            .ok()
            .flatten()
            .and_then(|bytes| bytes.as_ref().try_into().ok())
            .map(i128::from_be_bytes)
            .unwrap_or(i128::MIN)
    }

    pub fn set(&self, user_id: UserId, value: i128) -> Result<(), Error> {
        self.db.insert(user_id.get().to_be_bytes(), &value.to_be_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    pub fn all(&self) -> impl Iterator<Item = (UserId, i128)> + '_ {
        self.db.iter().filter_map(|res| {
            let (k, v) = res.ok()?;
            let uid = u64::from_be_bytes(k.as_ref().try_into().ok()?);
            let val = i128::from_be_bytes(v.as_ref().try_into().ok()?);
            Some((UserId::new(uid), val))
        })
    }
}
