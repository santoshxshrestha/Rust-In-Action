//link: https://z-galaxy.github.io/zbus/client.html
// busctl --user call \
//   org.freedesktop.Notifications \
//   /org/freedesktop/Notifications \
//   org.freedesktop.Notifications \
//   Notify \
//   susssasa\{sv\}i \
//   "my-app" 0 "dialog-information" "A summary" "Some body" 0 0 5000

// use std::collections::HashMap;
// use std::error::Error;
// use zbus::Connection;
// use zbus::zvariant::Value;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let connection = Connection::session().await?;
//     let m = connection
//         .call_method(
//             Some("org.freedesktop.Notifications"),
//             "/org/freedesktop/Notifications",
//             Some("org.freedesktop.Notifications"),
//             "Notify",
//             &(
//                 "my-app",
//                 0u32,
//                 "dialog-information",
//                 "A summary",
//                 "Some body",
//                 vec![""; 0],
//                 HashMap::<&str, &Value>::new(),
//                 5000,
//             ),
//         )
//         .await?;
//     let reply: u32 = m.body().deserialize::<u32>().unwrap();
//     dbg!(reply);
//     Ok(())
// }

// use std::collections::HashMap;
// use std::error::Error;
//
// use zbus::{Connection, proxy, zvariant::Value};
//
// #[proxy(
//     default_service = "org.freedesktop.Notifications",
//     default_path = "/org/freedesktop/Notifications"
// )]
// trait Notifications {
//     /// Call the org.freedesktop.Notifications.Notify D-Bus method
//     fn notify(
//         &self,
//         app_name: &str,
//         replaces_id: u32,
//         app_icon: &str,
//         summary: &str,
//         body: &str,
//         actions: &[&str],
//         hints: HashMap<&str, &Value<'_>>,
//         expire_timeout: i32,
//     ) -> zbus::Result<u32>;
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let connection = Connection::session().await?;
//
//     let proxy = NotificationsProxy::new(&connection).await?;
//     let reply = proxy
//         .notify(
//             "my-app",
//             0,
//             "dialog-information",
//             "A summary",
//             "Some body",
//             &[],
//             HashMap::new(),
//             5000,
//         )
//         .await?;
//     dbg!(reply);
//
//     Ok(())
// }
//

// use futures_util::stream::StreamExt;
// use zbus::{Connection, proxy, zvariant::OwnedObjectPath};
//
// #[proxy(
//     default_service = "org.freedesktop.systemd1",
//     default_path = "/org/freedesktop/systemd1",
//     interface = "org.freedesktop.systemd1.Manager"
// )]
// trait Systemd1Manager {
//     // Defines signature for D-Bus signal named `JobNew`
//     #[zbus(signal)]
//     fn job_new(&self, id: u32, job: OwnedObjectPath, unit: String) -> zbus::Result<()>;
// }
//
// async fn watch_systemd_jobs() -> zbus::Result<()> {
//     let connection = Connection::system().await?;
//     // `Systemd1ManagerProxy` is generated from `Systemd1Manager` trait
//     let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;
//     // Method `receive_job_new` is generated from `job_new` signal
//     let mut new_jobs_stream = systemd_proxy.receive_job_new().await?;
//
//     while let Some(msg) = new_jobs_stream.next().await {
//         // struct `JobNewArgs` is generated from `job_new` signal function arguments
//         let args: JobNewArgs = msg.args().expect("Error parsing message");
//
//         println!(
//             "JobNew received: unit={} id={} path={}",
//             args.unit, args.id, args.job
//         );
//     }
//
//     panic!("Stream ended unexpectedly");
// }use zbus::{zvariant::ObjectPath, proxy, Connection, Result};
// use futures_util::stream::StreamExt;
//
// #[proxy(
//     default_service = "org.freedesktop.GeoClue2",
//     interface = "org.freedesktop.GeoClue2.Manager",
//     default_path = "/org/freedesktop/GeoClue2/Manager"
// )]
// trait Manager {
//     #[zbus(object = "Client")]
//     fn get_client(&self);
// }
//
// #[proxy(
//     default_service = "org.freedesktop.GeoClue2",
//     interface = "org.freedesktop.GeoClue2.Client"
// )]
// trait Client {
//     fn start(&self) -> Result<()>;
//     fn stop(&self) -> Result<()>;
//
//     #[zbus(property)]
//     fn set_desktop_id(&mut self, id: &str) -> Result<()>;
//
//     #[zbus(signal)]
//     fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
// }
//
// #[proxy(
//     default_service = "org.freedesktop.GeoClue2",
//     interface = "org.freedesktop.GeoClue2.Location"
// )]
// trait Location {
//     #[zbus(property)]
//     fn latitude(&self) -> Result<f64>;
//     #[zbus(property)]
//     fn longitude(&self) -> Result<f64>;
// }
//
// Although we use `tokio` here, you can use any async runtime of choice.
// #[tokio::main]
// async fn main() -> Result<()> {
//     let conn = Connection::system().await?;
//     let manager = ManagerProxy::new(&conn).await?;
//     let mut client = manager.get_client().await?;
//     // Gotta do this, sorry!
//     client.set_desktop_id("org.freedesktop.zbus").await?;
//
//     let props = zbus::fdo::PropertiesProxy::builder(&conn)
//         .destination("org.freedesktop.GeoClue2")?
//         .path(client.inner().path())?
//         .build()
//         .await?;
//     let mut props_changed = props.receive_properties_changed().await?;
//     let mut location_updated = client.receive_location_updated().await?;
//
//     client.start().await?;
//
//     futures_util::try_join!(
//         async {
//             while let Some(signal) = props_changed.next().await {
//                 let args = signal.args()?;
//
//                 for (name, value) in args.changed_properties().iter() {
//                     println!("{}.{} changed to `{:?}`", args.interface_name(), name, value);
//                 }
//             }
//
//             Ok::<(), zbus::Error>(())
//         },
//         async {
//             while let Some(signal) = location_updated.next().await {
//                 let args = signal.args()?;
//
//                 let location = LocationProxy::builder(&conn)
//                     .path(args.new())?
//                     .build()
//                     .await?;
//                 println!(
//                     "Latitude: {}\nLongitude: {}",
//                     location.latitude().await?,
//                     location.longitude().await?,
//                 );
//             }
//
//             // No need to specify type of Result each time
//             Ok(())
//         }
//     )?;
//
//    Ok(())
// }

// #[tokio::main]
// async fn main() -> zbus::Result<()> {
//     let val = watch_systemd_jobs().await;
//     let _ = dbg!(val);
//     Ok(())
// }
use futures_util::stream::StreamExt;
use zbus::{Connection, Result, proxy, zvariant::ObjectPath};

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Manager",
    default_path = "/org/freedesktop/GeoClue2/Manager"
)]
trait Manager {
    #[zbus(object = "Client")]
    fn get_client(&self);
}

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Client"
)]
trait Client {
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;

    #[zbus(property)]
    fn set_desktop_id(&mut self, id: &str) -> Result<()>;

    #[zbus(signal)]
    fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
}

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Location"
)]
trait Location {
    #[zbus(property)]
    fn latitude(&self) -> Result<f64>;
    #[zbus(property)]
    fn longitude(&self) -> Result<f64>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::system().await?;
    let manager = ManagerProxy::new(&conn).await?;
    let mut client = manager.get_client().await?;
    // Gotta do this, sorry!
    client.set_desktop_id("org.freedesktop.zbus").await?;

    let props = zbus::fdo::PropertiesProxy::builder(&conn)
        .destination("org.freedesktop.GeoClue2")?
        .path(client.inner().path())?
        .build()
        .await?;
    let mut props_changed = props.receive_properties_changed().await?;
    let mut location_updated = client.receive_location_updated().await?;

    client.start().await?;

    futures_util::try_join!(
        async {
            while let Some(signal) = props_changed.next().await {
                let args = signal.args()?;

                for (name, value) in args.changed_properties().iter() {
                    println!(
                        "{}.{} changed to `{:?}`",
                        args.interface_name(),
                        name,
                        value
                    );
                }
            }

            Ok::<(), zbus::Error>(())
        },
        async {
            while let Some(signal) = location_updated.next().await {
                let args = signal.args()?;

                let location = LocationProxy::builder(&conn)
                    .path(args.new())?
                    .build()
                    .await?;
                println!(
                    "Latitude: {}\nLongitude: {}",
                    location.latitude().await?,
                    location.longitude().await?,
                );
            }

            // No need to specify type of Result each time
            Ok(())
        }
    )?;

    Ok(())
}
