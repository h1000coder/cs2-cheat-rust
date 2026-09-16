const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

// Mapeamento de virtual keys para nomes amigáveis
const keyNames = {
    // Mouse buttons
    0x01: 'LMB',
    0x02: 'RMB', 
    0x04: 'MMB',
    0x05: 'X1',
    0x06: 'X2',
    
    // Keyboard keys
    0x08: 'BSP',
    0x09: 'TAB',
    0x0D: 'ENT',
    0x10: 'SHIFT',
    0x11: 'CTRL',
    0x12: 'ALT',
    0x14: 'CAPS',
    0x20: 'SPC',
    0x21: 'PGUP',
    0x22: 'PGDN',
    0x23: 'END',
    0x24: 'HOME',
    0x25: 'LEFT',
    0x26: 'UP',
    0x27: 'RIGHT',
    0x28: 'DOWN',
    0x2C: 'PRTSC',
    0x2D: 'INS',
    0x2E: 'DEL',
    0x30: '0',
    0x31: '1',
    0x32: '2',
    0x33: '3',
    0x34: '4',
    0x35: '5',
    0x36: '6',
    0x37: '7',
    0x38: '8',
    0x39: '9',
    0x41: 'A',
    0x42: 'B',
    0x43: 'C',
    0x44: 'D',
    0x45: 'E',
    0x46: 'F',
    0x47: 'G',
    0x48: 'H',
    0x49: 'I',
    0x4A: 'J',
    0x4B: 'K',
    0x4C: 'L',
    0x4D: 'M',
    0x4E: 'N',
    0x4F: 'O',
    0x50: 'P',
    0x51: 'Q',
    0x52: 'R',
    0x53: 'S',
    0x54: 'T',
    0x55: 'U',
    0x56: 'V',
    0x57: 'W',
    0x58: 'X',
    0x59: 'Y',
    0x5A: 'Z',
    0x60: 'N0',
    0x61: 'N1',
    0x62: 'N2',
    0x63: 'N3',
    0x64: 'N4',
    0x65: 'N5',
    0x66: 'N6',
    0x67: 'N7',
    0x68: 'N8',
    0x69: 'N9',
    0x6A: 'N*',
    0x6B: 'N+',
    0x6C: 'N,',
    0x6D: 'N-',
    0x6E: 'N.',
    0x6F: 'N/',
    0x70: 'F1',
    0x71: 'F2',
    0x72: 'F3',
    0x73: 'F4',
    0x74: 'F5',
    0x75: 'F6',
    0x76: 'F7',
    0x77: 'F8',
    0x78: 'F9',
    0x79: 'F10',
    0x7A: 'F11',
    0x7B: 'F12',
};

// Estados de captura
let isCapturingAimKey = false;
let isCapturingTriggerKey = false;
let currentAimKey = 0x01; // Default: LMB
let currentTriggerKey = 0x02; // Default: RMB

// Elementos AIM
const aimElements = {
    aimbot_enabled: document.getElementById('aimbot_enabled'),
    aimbot_smooth: document.getElementById('aimbot_smooth'),
    aimbot_smooth_val: document.getElementById('aimbot_smooth_val'),
    aimbot_key_btn: document.getElementById('aimbot_key_btn'),
    aimbot_key_status: document.getElementById('aimbot_key_status'),
    trigger_bot_enabled: document.getElementById('trigger_bot_enabled'),
    trigger_delay: document.getElementById('trigger_delay'),
    trigger_delay_val: document.getElementById('trigger_delay_val'),
    trigger_hold_mode: document.getElementById('trigger_hold_mode'),
    trigger_key_btn: document.getElementById('trigger_key_btn'),
    trigger_key_status: document.getElementById('trigger_key_status'),
    fov_enabled: document.getElementById('fov_enabled'),
    fov_size: document.getElementById('fov_size'),
    fov_size_val: document.getElementById('fov_size_val'),
    fov_color: document.getElementById('fov_color')
};

// Elementos ESP
const espElements = {
    esp_enabled: document.getElementById('esp_enabled'),
    esp_box_enabled: document.getElementById('esp_box_enabled'),
    esp_filled_box_enabled: document.getElementById('esp_filled_box_enabled'),
    box_thickness: document.getElementById('box_thickness'),
    box_thickness_val: document.getElementById('box_thickness_val'),
    esp_name_enabled: document.getElementById('esp_name_enabled'),
    esp_health_bar_enabled: document.getElementById('esp_health_bar_enabled'),
    health_bar_width: document.getElementById('health_bar_width'),
    health_bar_width_val: document.getElementById('health_bar_width_val'),
    esp_distance_enabled: document.getElementById('esp_distance_enabled'),
    esp_skeleton_enabled: document.getElementById('esp_skeleton_enabled'),
    skeleton_thickness: document.getElementById('skeleton_thickness'),
    skeleton_thickness_val: document.getElementById('skeleton_thickness_val')
};

// Elementos CORES
const colorElements = {
    box_color_ct: document.getElementById('box_color_ct'),
    fill_box_color_ct: document.getElementById('fill_box_color_ct'),
    name_color_ct: document.getElementById('name_color_ct'),
    skeleton_color_ct: document.getElementById('skeleton_color_ct'),
    box_color_t: document.getElementById('box_color_t'),
    fill_box_color_t: document.getElementById('fill_box_color_t'),
    name_color_t: document.getElementById('name_color_t'),
    skeleton_color_t: document.getElementById('skeleton_color_t'),
    health_high: document.getElementById('health_high'),
    health_medium: document.getElementById('health_medium'),
    health_low: document.getElementById('health_low')
};

// Utilitários de cores
function hexToRgb(hex) {
    return [parseInt(hex.slice(1, 3), 16) / 255, parseInt(hex.slice(3, 5), 16) / 255, parseInt(hex.slice(5, 7), 16) / 255, 1.0];
}

function rgbToHex(rgb) {
    return '#' + Math.round(rgb[0] * 255).toString(16).padStart(2, '0') +
        Math.round(rgb[1] * 255).toString(16).padStart(2, '0') +
        Math.round(rgb[2] * 255).toString(16).padStart(2, '0');
}

// Função para atualizar o status da tecla
function updateKeybindStatus(element, keyName, isSet = true) {
    if (!element) return;
    if (isSet) {
        element.textContent = keyName;
        element.className = 'keybind-status set';
    } else if (keyName === 'waiting') {
        element.textContent = '...';
        element.className = 'keybind-status waiting';
    } else {
        element.textContent = 'None';
        element.className = 'keybind-status';
    }
}

// Setup da captura de tecla para Aimbot
function setupAimKeyCapture() {
    const aimKeyBtn = aimElements.aimbot_key_btn;
    const aimKeyStatus = aimElements.aimbot_key_status;
    
    if (!aimKeyBtn) return;
    
    aimKeyBtn.addEventListener('click', () => {
        isCapturingAimKey = true;
        aimKeyBtn.classList.add('recording');
        aimKeyBtn.textContent = '...';
        updateKeybindStatus(aimKeyStatus, 'waiting', false);
    });
}

// Setup da captura de tecla para Trigger Bot
function setupTriggerKeyCapture() {
    const triggerKeyBtn = aimElements.trigger_key_btn;
    const triggerKeyStatus = aimElements.trigger_key_status;
    
    if (!triggerKeyBtn) return;
    
    triggerKeyBtn.addEventListener('click', () => {
        isCapturingTriggerKey = true;
        triggerKeyBtn.classList.add('recording');
        triggerKeyBtn.textContent = '...';
        updateKeybindStatus(triggerKeyStatus, 'waiting', false);
    });
}

// Capturar teclas globalmente
function setupGlobalKeyCapture() {
    // Capturar teclas do teclado
    document.addEventListener('keydown', (e) => {
        // Capturar para Aimbot
        if (isCapturingAimKey) {
            e.preventDefault();
            let keyCode = e.keyCode || e.which;
            let keyName = keyNames[keyCode] || keyCode.toString(16).toUpperCase();
            
            currentAimKey = keyCode;
            updateKeybindStatus(aimElements.aimbot_key_status, keyName, true);
            aimElements.aimbot_key_btn.classList.remove('recording');
            aimElements.aimbot_key_btn.textContent = 'Set';
            isCapturingAimKey = false;
            saveConfig();
            return;
        }
        
        // Capturar para Trigger Bot
        if (isCapturingTriggerKey) {
            e.preventDefault();
            let keyCode = e.keyCode || e.which;
            let keyName = keyNames[keyCode] || keyCode.toString(16).toUpperCase();
            
            currentTriggerKey = keyCode;
            updateKeybindStatus(aimElements.trigger_key_status, keyName, true);
            aimElements.trigger_key_btn.classList.remove('recording');
            aimElements.trigger_key_btn.textContent = 'Set';
            isCapturingTriggerKey = false;
            saveConfig();
            return;
        }
    });
    
    // Capturar botões do mouse
    document.addEventListener('mousedown', (e) => {
        // Capturar para Aimbot
        if (isCapturingAimKey) {
            e.preventDefault();
            let keyCode = 0;
            let keyName = '';
            
            switch(e.button) {
                case 0: keyCode = 0x01; keyName = 'LMB'; break;
                case 1: keyCode = 0x04; keyName = 'MMB'; break;
                case 2: keyCode = 0x02; keyName = 'RMB'; break;
                case 3: keyCode = 0x05; keyName = 'X1'; break;
                case 4: keyCode = 0x06; keyName = 'X2'; break;
                default: return;
            }
            
            currentAimKey = keyCode;
            updateKeybindStatus(aimElements.aimbot_key_status, keyName, true);
            aimElements.aimbot_key_btn.classList.remove('recording');
            aimElements.aimbot_key_btn.textContent = 'Set';
            isCapturingAimKey = false;
            saveConfig();
            return;
        }
        
        // Capturar para Trigger Bot
        if (isCapturingTriggerKey) {
            e.preventDefault();
            let keyCode = 0;
            let keyName = '';
            
            switch(e.button) {
                case 0: keyCode = 0x01; keyName = 'LMB'; break;
                case 1: keyCode = 0x04; keyName = 'MMB'; break;
                case 2: keyCode = 0x02; keyName = 'RMB'; break;
                case 3: keyCode = 0x05; keyName = 'X1'; break;
                case 4: keyCode = 0x06; keyName = 'X2'; break;
                default: return;
            }
            
            currentTriggerKey = keyCode;
            updateKeybindStatus(aimElements.trigger_key_status, keyName, true);
            aimElements.trigger_key_btn.classList.remove('recording');
            aimElements.trigger_key_btn.textContent = 'Set';
            isCapturingTriggerKey = false;
            saveConfig();
            return;
        }
    });
}

// Salvar configuração completa
async function saveConfig() {
    const config = {
        aimbot_enabled: aimElements.aimbot_enabled?.checked || false,
        aimbot_smooth: parseFloat(aimElements.aimbot_smooth?.value || 5.0),
        aimbot_key: currentAimKey,
        trigger_bot_enabled: aimElements.trigger_bot_enabled?.checked || false,
        trigger_delay_ms: parseInt(aimElements.trigger_delay?.value || 0),
        trigger_hold_mode: aimElements.trigger_hold_mode?.checked || true,
        trigger_key: currentTriggerKey,
        trigger_team_check: true,
        trigger_scan_delay_ms: 1,
        fov_enabled: aimElements.fov_enabled?.checked || true,
        fov_size: parseFloat(aimElements.fov_size?.value || 100),
        fov_color: hexToRgb(aimElements.fov_color?.value || '#ffffff'),
        
        esp_enabled: espElements.esp_enabled?.checked !== false,
        esp_box_enabled: espElements.esp_box_enabled?.checked !== false,
        esp_health_bar_enabled: espElements.esp_health_bar_enabled?.checked !== false,
        esp_name_enabled: espElements.esp_name_enabled?.checked !== false,
        esp_distance_enabled: espElements.esp_distance_enabled?.checked !== false,
        esp_skeleton_enabled: espElements.esp_skeleton_enabled?.checked || false,
        esp_filled_box_enabled: espElements.esp_filled_box_enabled?.checked || false,
        
        box_color_ct: hexToRgb(colorElements.box_color_ct?.value || '#0080ff'),
        fill_box_color_ct: hexToRgb(colorElements.fill_box_color_ct?.value || '#0080ff'),
        box_color_t: hexToRgb(colorElements.box_color_t?.value || '#ff3300'),
        fill_box_color_t: hexToRgb(colorElements.fill_box_color_t?.value || '#ff3300'),
        name_color_ct: hexToRgb(colorElements.name_color_ct?.value || '#80b3ff'),
        name_color_t: hexToRgb(colorElements.name_color_t?.value || '#ff8066'),
        skeleton_color_ct: hexToRgb(colorElements.skeleton_color_ct?.value || '#0080ff'),
        skeleton_color_t: hexToRgb(colorElements.skeleton_color_t?.value || '#ff3300'),
        
        health_high_color: hexToRgb(colorElements.health_high?.value || '#00ff00'),
        health_medium_color: hexToRgb(colorElements.health_medium?.value || '#ffff00'),
        health_low_color: hexToRgb(colorElements.health_low?.value || '#ff0000'),
        
        health_bar_width: parseFloat(espElements.health_bar_width?.value || 6),
        box_thickness: parseFloat(espElements.box_thickness?.value || 2),
        skeleton_thickness: parseFloat(espElements.skeleton_thickness?.value || 1.5),
        head_offset_y: -12.0
    };

    try {
        await invoke('update_cheat_config', { config });
        console.log("Configuração salva:", config);
    } catch (err) {
        console.error("Erro ao salvar config:", err);
    }
}

// Carregar configuração do Rust
async function loadConfig() {
    const cfg = await invoke('get_cheat_config');

    // AIM settings
    if (aimElements.aimbot_enabled) aimElements.aimbot_enabled.checked = cfg.aimbot_enabled || false;
    if (aimElements.aimbot_smooth) {
        aimElements.aimbot_smooth.value = cfg.aimbot_smooth || 5;
        if (aimElements.aimbot_smooth_val) aimElements.aimbot_smooth_val.textContent = cfg.aimbot_smooth || 5;
    }
    
    // Carregar aim key
    if (cfg.aimbot_key) {
        currentAimKey = cfg.aimbot_key;
        let keyName = keyNames[currentAimKey] || currentAimKey.toString(16).toUpperCase();
        updateKeybindStatus(aimElements.aimbot_key_status, keyName, true);
    }
    
    if (aimElements.trigger_bot_enabled) aimElements.trigger_bot_enabled.checked = cfg.trigger_bot_enabled || false;
    if (aimElements.trigger_hold_mode) aimElements.trigger_hold_mode.checked = cfg.trigger_hold_mode !== false;
    if (aimElements.trigger_delay) {
        aimElements.trigger_delay.value = cfg.trigger_delay_ms || 0;
        if (aimElements.trigger_delay_val) aimElements.trigger_delay_val.textContent = cfg.trigger_delay_ms || 0;
    }
    
    // Carregar trigger key
    if (cfg.trigger_key) {
        currentTriggerKey = cfg.trigger_key;
        let keyName = keyNames[currentTriggerKey] || currentTriggerKey.toString(16).toUpperCase();
        updateKeybindStatus(aimElements.trigger_key_status, keyName, true);
    }
    
    if (aimElements.fov_enabled) aimElements.fov_enabled.checked = cfg.fov_enabled !== false;
    if (aimElements.fov_size) {
        aimElements.fov_size.value = cfg.fov_size || 100;
        if (aimElements.fov_size_val) aimElements.fov_size_val.textContent = cfg.fov_size || 100;
    }
    if (aimElements.fov_color) aimElements.fov_color.value = rgbToHex(cfg.fov_color || [1.0, 1.0, 1.0, 1.0]);

    // ESP settings
    if (espElements.esp_enabled) espElements.esp_enabled.checked = cfg.esp_enabled !== false;
    if (espElements.esp_box_enabled) espElements.esp_box_enabled.checked = cfg.esp_box_enabled !== false;
    if (espElements.esp_health_bar_enabled) espElements.esp_health_bar_enabled.checked = cfg.esp_health_bar_enabled !== false;
    if (espElements.esp_name_enabled) espElements.esp_name_enabled.checked = cfg.esp_name_enabled !== false;
    if (espElements.esp_distance_enabled) espElements.esp_distance_enabled.checked = cfg.esp_distance_enabled !== false;
    if (espElements.esp_skeleton_enabled) espElements.esp_skeleton_enabled.checked = cfg.esp_skeleton_enabled || false;
    if (espElements.esp_filled_box_enabled) espElements.esp_filled_box_enabled.checked = cfg.esp_filled_box_enabled || false;
    if (espElements.box_thickness) {
        espElements.box_thickness.value = cfg.box_thickness || 2;
        if (espElements.box_thickness_val) espElements.box_thickness_val.textContent = cfg.box_thickness || 2;
    }
    if (espElements.health_bar_width) {
        espElements.health_bar_width.value = cfg.health_bar_width || 6;
        if (espElements.health_bar_width_val) espElements.health_bar_width_val.textContent = cfg.health_bar_width || 6;
    }
    if (espElements.skeleton_thickness) {
        espElements.skeleton_thickness.value = cfg.skeleton_thickness || 1.5;
        if (espElements.skeleton_thickness_val) espElements.skeleton_thickness_val.textContent = cfg.skeleton_thickness || 1.5;
    }

    // Color settings
    if (colorElements.box_color_ct) colorElements.box_color_ct.value = rgbToHex(cfg.box_color_ct || [0.0, 0.5, 1.0, 1.0]);
    if (colorElements.box_color_t) colorElements.box_color_t.value = rgbToHex(cfg.box_color_t || [1.0, 0.2, 0.0, 1.0]);
    if (colorElements.name_color_ct) colorElements.name_color_ct.value = rgbToHex(cfg.name_color_ct || [0.5, 0.7, 1.0, 1.0]);
    if (colorElements.name_color_t) colorElements.name_color_t.value = rgbToHex(cfg.name_color_t || [1.0, 0.5, 0.3, 1.0]);
    if (colorElements.skeleton_color_ct) colorElements.skeleton_color_ct.value = rgbToHex(cfg.skeleton_color_ct || [0.0, 0.5, 1.0, 0.8]);
    if (colorElements.skeleton_color_t) colorElements.skeleton_color_t.value = rgbToHex(cfg.skeleton_color_t || [1.0, 0.2, 0.0, 0.8]);
    if (colorElements.health_high) colorElements.health_high.value = rgbToHex(cfg.health_high_color || [0.0, 1.0, 0.0, 0.9]);
    if (colorElements.health_medium) colorElements.health_medium.value = rgbToHex(cfg.health_medium_color || [1.0, 1.0, 0.0, 0.9]);
    if (colorElements.health_low) colorElements.health_low.value = rgbToHex(cfg.health_low_color || [1.0, 0.0, 0.0, 0.9]);
    if (colorElements.fill_box_color_ct) colorElements.fill_box_color_ct.value = rgbToHex(cfg.fill_box_color_ct || [0.0, 0.5, 1.0, 0.3]);
    if (colorElements.fill_box_color_t) colorElements.fill_box_color_t.value = rgbToHex(cfg.fill_box_color_t || [1.0, 0.2, 0.0, 0.3]);
}

// Resetar configurações
async function resetConfig() {
    await invoke('reset_cheat_config');
    currentAimKey = 0x02;
    currentTriggerKey = 0x01;
    await loadConfig();
}

// Atualizar informações do jogo
async function updateGameInfo() {
    const [health, team, enemies] = await invoke('get_game_data');
    document.getElementById('stat_health').textContent = health;
    document.getElementById('stat_team').textContent = team === 2 ? 'CT' : 'T';
    document.getElementById('stat_enemies').textContent = enemies.length;
    document.getElementById('game_status').innerHTML = enemies.length > 0 ? '● IN GAME' : '● ONLINE';
    if (enemies.length > 0) {
        document.getElementById('stat_closest').textContent = enemies[0][0].substring(0, 12);
    } else {
        document.getElementById('stat_closest').textContent = '-';
    }
}

// Toggle window visibility com INSERT
async function toggleWindow() {
    try {
        const window = getCurrentWindow();
        const isVisible = await window.isVisible();
        
        if (isVisible) {
            await window.hide();
            console.log('Window hidden - Press INSERT to show');
        } else {
            await window.show();
            await window.setAlwaysOnTop(true);
            await window.setFocus();
            console.log('Window shown and set on top');
        }
    } catch (err) {
        console.error('Error toggling window:', err);
    }
}

// Adicionar event listeners
function addEventListeners() {
    // AIM elements
    for (const [id, el] of Object.entries(aimElements)) {
        if (el && el.type !== 'range' && id !== 'aimbot_key_btn' && id !== 'aimbot_key_status' && id !== 'trigger_key_btn' && id !== 'trigger_key_status') {
            el.addEventListener('change', saveConfig);
        } else if (el && el.type === 'range') {
            el.addEventListener('input', (e) => {
                const valId = `${el.id}_val`;
                if (aimElements[valId]) aimElements[valId].textContent = e.target.value;
                saveConfig();
            });
        }
    }

    // ESP elements
    for (const [id, el] of Object.entries(espElements)) {
        if (el && el.type !== 'range') {
            el.addEventListener('change', saveConfig);
        } else if (el && el.type === 'range') {
            el.addEventListener('input', (e) => {
                const valId = `${el.id}_val`;
                if (espElements[valId]) espElements[valId].textContent = e.target.value;
                saveConfig();
            });
        }
    }

    // Color elements
    for (const el of Object.values(colorElements)) {
        if (el) {
            el.addEventListener('change', saveConfig);
        }
    }

    // Reset button
    const resetBtn = document.getElementById('reset_btn');
    if (resetBtn) resetBtn.addEventListener('click', resetConfig);
}

// Configurar hotkey INSERT
function setupHotkey() {
    // Evento keydown padrão
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Insert' || e.keyCode === 45) {
            e.preventDefault();
            e.stopPropagation();
            toggleWindow();
            return false;
        }
        
        // END key para fechar
        if (e.key === 'End' || e.keyCode === 35) {
            e.preventDefault();
            const window = getCurrentWindow();
            window.close();
        }
    });
    
    // Evento keyup para garantir
    document.addEventListener('keyup', (e) => {
        if (e.key === 'Insert' || e.keyCode === 45) {
            e.preventDefault();
            e.stopPropagation();
        }
    });
    
    // Listener global na janela
    window.addEventListener('keydown', (e) => {
        if (e.key === 'Insert' || e.keyCode === 45) {
            e.preventDefault();
            toggleWindow();
        }
    });
    
    console.log('✅ Hotkey INSERT configured - Press INSERT to show/hide window');
}

// Garantir que a janela inicie no topo
async function initWindow() {
    try {
        const window = getCurrentWindow();
        await window.setAlwaysOnTop(true);
        console.log('✅ Window always on top enabled');
        
        // Criar indicador visual
        const indicator = document.createElement('div');
        indicator.style.cssText = `
            position: fixed;
            bottom: 5px;
            right: 5px;
            background: #00ff66;
            color: #000;
            font-size: 9px;
            font-family: monospace;
            padding: 2px 6px;
            border-radius: 2px;
            z-index: 9999;
            opacity: 0.7;
        `;
        indicator.textContent = 'INSERT';
        document.body.appendChild(indicator);
    } catch (err) {
        console.error('Error setting always on top:', err);
    }
}

// Tab navigation
document.querySelectorAll('.nav-item').forEach(item => {
    item.addEventListener('click', () => {
        const tab = item.dataset.tab;
        document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        item.classList.add('active');
        document.getElementById(`tab-${tab}`).classList.add('active');
    });
});

// Inicialização completa
async function init() {
    await loadConfig();
    updateGameInfo();
    addEventListeners();
    setupAimKeyCapture();
    setupTriggerKeyCapture();
    setupGlobalKeyCapture();
    setupHotkey();
    await initWindow();
}

// Start
init();
setInterval(updateGameInfo, 1000);