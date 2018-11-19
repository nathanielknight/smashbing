class AudioPlayer {
  constructor() {
    // BufferLoader to load sounds?
    //
    this.buffers = new Object();
    this.ctx = new AudioContext();
    this.gainNode = this.ctx.createGain();
    this.gainNode.connect(this.ctx.destination);
  }

  // Load a file over the network
  async loadAudio(url) {
    let req = new Request(url);
    req.headers.append("Content-Type", "audio/wav");
    let resp = await fetch(req);
    let audioData = await resp.arrayBuffer();
    let buff = await this.ctx.decodeAudioData(audioData);
    return buff;
  }

  // Load audio file and save it in a buffer
  async addAudio(name, url) {
    let buff = await this.loadAudio(url);
    this.buffers[name] = buff;
  }

  // Create an audio source from a sound effect's audio buffer
  playSound(name) {
    let buff = this.buffers[name];
    if (buff == null) {
      return;
    }
    let source = this.ctx.createBufferSource();
    source.buffer = buff;
    source.connect(this.gainNode);
    source.start();
  }
}
