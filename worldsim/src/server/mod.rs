pub mod meta;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread::sleep;
use std::time::Duration;

use crate::{
    job::JobManager, region::Region, regionmanager::meta::RegionManagerMsg, server::meta::RegionId,
    server::meta::ServerMsg,
};

/*
one server per physical host
*/
pub struct Server {
    tx: mpsc::Sender<ServerMsg>,
    rx: mpsc::Receiver<RegionManagerMsg>,
    running: Arc<AtomicBool>,
    id: Option<u64>,
    seed: Option<u64>,
    state: u64,
    _jobmanager: Arc<JobManager>,
    _region: HashMap<RegionId, Region>,
}

impl Server {
    pub fn new(
        tx: mpsc::Sender<ServerMsg>,
        rx: mpsc::Receiver<RegionManagerMsg>,
        jobmanager: Arc<JobManager>,
    ) -> Self {
        let running = Arc::new(AtomicBool::new(true));

        Self {
            tx,
            rx,
            running,
            id: None,
            seed: None,
            state: 0,
            _jobmanager: jobmanager.clone(),
            _region: HashMap::new(),
        }
    }

    pub fn work(
        &mut self,
        //jm: &JobManager,
    ) -> bool {
        match self.state {
            0 => {
                self.tx.send(ServerMsg::Attach()).unwrap();
                self.state += 1;
            }
            _ => (),
        }

        match self.rx.try_recv() {
            Ok(msg) => {
                match msg {
                    RegionManagerMsg::Attached { server_id, seed } => {
                        self.id = Some(server_id);
                        self.seed = Some(seed);
                    }
                    RegionManagerMsg::NewServerInMesh {
                        server_id,
                        server_connection_details,
                    } => {
                        println!(
                            "new server found {}, details: {:?}",
                            server_id, server_connection_details
                        );
                    }
                    RegionManagerMsg::CreateRegion { region_id } => {
                        /*
                        let mut r = Region::new(region_id, self.jobmanager.clone());
                        r.block.make_at_least(Vec3::new(0,0,0), Vec3::new(65535,65535,65535), 9);
                        self.region.insert(region_id, r);
                        */
                        println!("create region {:?}", region_id);
                    }
                    RegionManagerMsg::TakeOverRegionFrom {
                        region_id,
                        server_id,
                    } => {
                        println!(
                            "new server in mesh, region: {:?}, server {}",
                            region_id, server_id
                        );
                    } //_ => (),
                }
            }
            Err(e) => {
                debug!("Work error {:?}", e);
                sleep(Duration::from_millis(10));
            }
        }

        self.running.load(Ordering::Relaxed)
    }
}
