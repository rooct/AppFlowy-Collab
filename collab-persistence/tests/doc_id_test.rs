use std::ops::{Range, RangeTo};
use std::sync::Once;

use collab_persistence::CollabKV;

use tempfile::TempDir;
use tracing_subscriber::{fmt::Subscriber, util::SubscriberInitExt, EnvFilter};

#[test]
fn id_test() {
  let db = db();
  db.insert([0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 1]).unwrap();
  db.insert([0, 0, 0, 0, 0, 0, 0, 1], &[0, 1, 2]).unwrap();
  db.insert([0, 0, 0, 0, 0, 0, 0, 2], &[0, 1, 3]).unwrap();
  db.insert([0, 0, 0, 0, 0, 0, 0, 3], &[0, 1, 4]).unwrap();
  db.insert([0, 1, 0, 0, 0, 0, 0, 4], &[0, 1, 5]).unwrap();
  db.insert([0, 1, 0, 0, 0, 0, 0, 5], &[0, 1, 6]).unwrap();

  let given_key: &[u8; 8] = &[0, 0, 0, 0, 0, 0, 0, 1];
  let last_entry_prior = db
      .range::<&[u8;8],RangeTo<&[u8;8]>>(..given_key) // Create a range up to (excluding) the given key
      .next_back() // Get the last entry in the range
      .expect("No entry found prior to the given key").unwrap();
  println!("{:?}", last_entry_prior.1);

  let given_key: &[u8; 2] = &[0, 1];
  let last_entry_prior = db
      .range::<&[u8;2],RangeTo<&[u8;2]>>(..given_key) // Create a range up to (excluding) the given key
      .next_back() // Get the last entry in the range
      .expect("No entry found prior to the given key").unwrap();
  println!("{:?}", last_entry_prior.1);

  let prefix: &[u8] = &[0, 1, 0, 0, 0, 0, 0];
  let mut r = db.scan_prefix(prefix);
  println!("{:?}", r.next_back())
}

#[test]
fn range_key_test() {
  let db = db();
  db.insert([0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 1]).unwrap();
  db.insert([0, 0, 0, 0, 0, 0, 0, 1], &[0, 1, 2]).unwrap();
  db.insert([0, 0, 0, 0, 0, 0, 0, 2], &[0, 1, 3]).unwrap();

  db.insert([0, 0, 1, 0, 0, 0, 0, 0], &[0, 2, 1]).unwrap();
  db.insert([0, 0, 1, 0, 0, 0, 0, 1], &[0, 2, 2]).unwrap();
  db.insert([0, 0, 1, 0, 0, 0, 0, 2], &[0, 2, 3]).unwrap();

  db.insert([0, 0, 2, 0, 0, 0, 0, 0], &[0, 3, 1]).unwrap();
  db.insert([0, 0, 2, 0, 0, 0, 0, 1], &[0, 3, 2]).unwrap();
  db.insert([0, 0, 2, 0, 0, 0, 0, 2], &[0, 3, 3]).unwrap();

  db.insert([0, 1, 0, 0, 0, 0, 0, 3], &[0, 1, 4]).unwrap();
  db.insert([0, 1, 0, 0, 0, 0, 0, 4], &[0, 1, 5]).unwrap();
  db.insert([0, 1, 0, 0, 0, 0, 0, 5], &[0, 1, 6]).unwrap();

  let given_key: &[u8; 8] = &[0, 0, 0, 0, 0, 0, 0, u8::MAX];
  let mut iter = db.range::<&[u8; 8], RangeTo<&[u8; 8]>>(..given_key);
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 0, 0, 0, 0, 0, 0]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 0, 0, 0, 0, 0, 1]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 0, 0, 0, 0, 0, 2]
  );
  assert!(iter.next().is_none());

  let start: &[u8; 8] = &[0, 0, 1, 0, 0, 0, 0, 0];
  let given_key: &[u8; 8] = &[0, 0, 1, 0, 0, 0, 0, u8::MAX];
  let mut iter = db.range::<&[u8; 8], Range<&[u8; 8]>>(start..given_key);
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 1, 0, 0, 0, 0, 0]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 1, 0, 0, 0, 0, 1]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 0, 1, 0, 0, 0, 0, 2]
  );
  assert!(iter.next().is_none());

  let given_key: &[u8; 2] = &[0, 1];
  let last_entry_prior = db
      .range::<&[u8;2],RangeTo<&[u8;2]>>(..given_key) // Create a range up to (excluding) the given key
      .next_back()
      .expect("No entry found prior to the given key").unwrap();
  assert_eq!(last_entry_prior.1.as_ref(), &[0, 3, 3]);

  let prefix: &[u8] = &[0, 1, 0, 0, 0, 0, 0];
  let mut r = db.scan_prefix(prefix);
  println!("{:?}", r.next_back());

  let start: &[u8; 8] = &[0, 1, 0, 0, 0, 0, 0, 3];
  let given_key: &[u8; 8] = &[0, 1, 0, 0, 0, 0, 0, u8::MAX];
  let mut iter = db.range::<&[u8; 8], Range<&[u8; 8]>>(start..given_key);
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 1, 0, 0, 0, 0, 0, 3]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 1, 0, 0, 0, 0, 0, 4]
  );
  assert_eq!(
    iter.next().unwrap().unwrap().0.as_ref(),
    &[0, 1, 0, 0, 0, 0, 0, 5]
  );
  assert!(iter.next().is_none());

  // for i in 0..10 {
  //   let oid = format!("block_{}", i);
  //   let doc = Doc::new();
  //   let txn = doc.transact();
  //   db.doc(1).create_new_doc(&oid, &txn).unwrap();
  // }
}

fn db() -> CollabKV {
  static START: Once = Once::new();
  START.call_once(|| {
    std::env::set_var("RUST_LOG", "collab_persistence=trace");
    let subscriber = Subscriber::builder()
      .with_env_filter(EnvFilter::from_default_env())
      .with_ansi(true)
      .finish();
    subscriber.try_init().unwrap();
  });

  let tempdir = TempDir::new().unwrap();
  let path = tempdir.into_path();
  CollabKV::open(path).unwrap()
}
