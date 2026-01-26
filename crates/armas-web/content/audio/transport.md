# Transport Control

DAW transport bar with playback controls and tempo.

```demo
let transport_id = ui.id().with("transport");
let transport: TransportControl = ui.ctx().data_mut(|d| {
    d.get_persisted(transport_id).unwrap_or_else(|| TransportControl::new().tempo(120.0).time_signature(4, 4))
});
let response = transport.show(ui, &theme);
```
