import React from "react";
import "./VolumeControl.css";

interface VolumeControlProps {
  isOpen: boolean;
  onClose: () => void;
  bgVolume: number;
  sfxVolume: number;
  onBgVolumeChange: (volume: number) => void;
  onSfxVolumeChange: (volume: number) => void;
}

const VolumeControl: React.FC<VolumeControlProps> = ({
  isOpen,
  onClose,
  bgVolume,
  sfxVolume,
  onBgVolumeChange,
  onSfxVolumeChange,
}) => {
  if (!isOpen) return null;

  return (
    <div className="volume-control-overlay" onClick={onClose}>
      <div className="volume-control-popup" onClick={(e) => e.stopPropagation()}>
        <div className="volume-control-header">
          <h2>音量設定</h2>
          <button className="volume-control-close" onClick={onClose}>
            ×
          </button>
        </div>

        <div className="volume-control-content">
          <div className="volume-slider-section">
            <label htmlFor="bg-volume">背景音樂</label>
            <div className="volume-slider-container">
              <input
                id="bg-volume"
                type="range"
                min="0"
                max="100"
                value={bgVolume * 100}
                onChange={(e) => onBgVolumeChange(Number(e.target.value) / 100)}
                className="volume-slider"
              />
              <span className="volume-value">{Math.round(bgVolume * 100)}%</span>
            </div>
          </div>

          <div className="volume-slider-section">
            <label htmlFor="sfx-volume">音效</label>
            <div className="volume-slider-container">
              <input
                id="sfx-volume"
                type="range"
                min="0"
                max="100"
                value={sfxVolume * 100}
                onChange={(e) => onSfxVolumeChange(Number(e.target.value) / 100)}
                className="volume-slider"
              />
              <span className="volume-value">{Math.round(sfxVolume * 100)}%</span>
            </div>
          </div>
        </div>

        <div className="volume-control-footer">
          <button className="btn-primary" onClick={onClose}>
            確定
          </button>
        </div>
      </div>
    </div>
  );
};

export default VolumeControl;
