// This class uses the WebAudio API to implement game-appropriate sound
// effects. In theory, this could be implemented by creating a bunch of `Audio`
// elements and calling `play` on them at the appropriate times, but in
// practice, sound effects played with that setup don't line up with the game
// events that they're supposed to line up with.

// https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API

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
