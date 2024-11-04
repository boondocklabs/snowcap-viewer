use async_trait::async_trait;
use snowcap::{
    iced::Task,
    module::{
        argument::ModuleArguments, data::ModuleData, error::ModuleError, event::ModuleEvent,
        Module, ModuleInitData,
    },
};

#[derive(Debug)]
pub struct MyModuleData {}
impl ModuleData for MyModuleData {
    fn kind(&self) -> snowcap::module::data::ModuleDataKind {
        todo!()
    }

    fn bytes(&self) -> Result<&Vec<u8>, ModuleError> {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct MyModule {}

#[derive(Debug)]
pub enum MyModuleEvent {
    Init,
}
impl ModuleEvent for MyModuleEvent {}

#[async_trait]
impl Module for MyModule {
    type Event = MyModuleEvent;
    type Data = MyModuleData;

    async fn init(
        &mut self,
        _args: ModuleArguments,
        _init_data: ModuleInitData,
    ) -> Result<MyModuleEvent, ModuleError> {
        println!("My Custom Module Async Init");
        Ok(MyModuleEvent::Init)
    }

    fn on_event(
        &mut self,
        _event: Self::Event,
    ) -> snowcap::iced::Task<snowcap::module::message::ModuleMessage> {
        println!("Custom module received event: {_event:?}");
        Task::none()
    }
}
