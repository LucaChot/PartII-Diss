use super::*;
use super::super::*;
use super::super::taurus::*;
use std::{thread::sleep, time::Instant};

#[test]
fn test_core_debug_time_progresses(){
  let network_builder = TimeTaurusNetworkBuilder::new(0, 1, 0);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    let t = Instant::now();
    while t.elapsed() < Duration::new(0,500000000) {
      continue
    }
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::LEFT);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::RIGHT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();

  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 4400);
  assert!(debug[0].stat.as_millis() < 4600);
  assert!(debug[1].stat.as_millis() > 4400);
  assert!(debug[1].stat.as_millis() < 4600);
}

#[test]
fn test_core_debug_time_handles_sleep(){
  let network_builder = TimeTaurusNetworkBuilder::new(2000000000,0,0);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    sleep(Duration::new(2, 0));

    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::LEFT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    core_info.recv(&TaurusOption::RIGHT);
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() < 6020);
  assert!(debug[0].stat.as_millis() > 5980);
  assert!(debug[1].stat.as_millis() > 3980);
  assert!(debug[1].stat.as_millis() < 4020);
}

#[test]
fn test_core_debug_time_received_is_less(){
  let network_builder = TimeTaurusNetworkBuilder::new(0, 1000000000, 0);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::LEFT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    let t = Instant::now();
    while t.elapsed() < Duration::new(0,500000000) {
      continue
    }
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::RIGHT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[0].row);
  dbg!(&debug[0].col);
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[1].stat.as_millis() < 10);
  assert!(debug[0].stat.as_millis() > 450);
  assert!(debug[0].stat.as_millis() < 550);
}

#[test]
fn test_comm_info_bandwidth_2ms(){
  let network_builder = TimeTaurusNetworkBuilder::new(0, 2, 0);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::LEFT);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::RIGHT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 1900);
  assert!(debug[0].stat.as_millis() < 2100);
  assert!(debug[1].stat.as_millis() > 1900);
  assert!(debug[1].stat.as_millis() < 2100);
}

#[test]
fn test_comm_info_latency(){
  let network_builder = TimeTaurusNetworkBuilder::new(200000000, 2, 0);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::LEFT);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::RIGHT);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 2100);
  assert!(debug[0].stat.as_millis() < 2300);
  assert!(debug[1].stat.as_millis() > 1980);
  assert!(debug[1].stat.as_millis() < 2020);
}

#[test]
fn test_comm_info_startup_2cores(){
  let network_builder = TimeTaurusNetworkBuilder::new(0, 1, 500000000);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::ROW);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::ROW);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 4900);
  assert!(debug[0].stat.as_millis() < 5100);
  assert!(debug[1].stat.as_millis() > 4900);
  assert!(debug[1].stat.as_millis() < 5100);
}

#[test]
fn test_comm_info_startup_2cores_with_latency(){
  let network_builder = TimeTaurusNetworkBuilder::new(200000000, 1, 500000000);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(2,2, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::ROW);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::ROW);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 5100);
  assert!(debug[0].stat.as_millis() < 5300);
  assert!(debug[1].stat.as_millis() > 4900);
  assert!(debug[1].stat.as_millis() < 5100);
}

#[test]
fn test_comm_info_startup_3cores(){
  let network_builder = TimeTaurusNetworkBuilder::new(0, 1, 500000000);
  let mut processor : ProbeProcessor <Duration, (),(i32,Duration), TimedTaurusCore<(i32,Duration)>> = 
    ProbeProcessor::new(3,3, network_builder);
  
  let p0 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.send(1, &TaurusOption::ROW);
  };

  let p1 = move |core_info: &mut ThreadTimeProber<i32, TimedTaurusCore<(i32,Duration)>>| {
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
    core_info.recv(&TaurusOption::ROW);
    dbg!(&core_info.probe.get_curr_elapsed().as_millis());
  };

  processor.run_core(p0);
  processor.run_core(p1);
  
  processor.collect_results();
  let debug = processor.debug_stats();
  
  dbg!(&debug[0].stat.as_millis());
  dbg!(&debug[1].stat.as_millis());

  assert!(debug[0].stat.as_millis() > 5400);
  assert!(debug[0].stat.as_millis() < 5600);
  assert!(debug[1].stat.as_millis() > 5400);
  assert!(debug[1].stat.as_millis() < 5600);
}
