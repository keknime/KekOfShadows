import init, { Game } from '/wasm_game.js';

async function initGame() {
    await init();
    const game = new Game();
    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');
    const combatLog = document.getElementById('combat-log');
    const skillsMenu = document.getElementById('skills-menu');
    const equipmentWindow = document.getElementById('equipment-window');
    const skillTreeControls = document.getElementById('skill-tree-controls');
    const playerInfo = document.getElementById('player-info');
    const itemControls = document.getElementById('item-controls');
    let showingSkillTree = false;
    let wallet = null;
    let ws = null;

    async function loadGameData() {
        const response = await fetch('/game_data.json');
        const data = await response.json();
        game.load_game_data(JSON.stringify(data));
    }

    async function loadMap(location) {
        const response = await fetch(`/maps/${location}.json`);
        const data = await response.json();
        game.load_map(location, JSON.stringify(data));
    }

    await loadGameData();
    await loadMap('Town');

    function connectWebSocket(walletAddress) {
        ws = new WebSocket(`ws://localhost:8080/${walletAddress}`);
        ws.onopen = () => {
            console.log('WebSocket connected');
        };
        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.type === 'players') {
                game.update_other_players(JSON.stringify(data.players));
                if (!showingSkillTree) {
                    renderGame();
                }
            }
        };
        ws.onclose = ($
event) => {
            console.log('WebSocket disconnected');
        };
    }

    function sendPlayerUpdate(character) {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({
                type: 'update',
                wallet: wallet,
                name: character.name,
                level: character.level,
                x: character.x,
                y: character.y,
                location: character.location,
                equipment: character.equipment,
            }));
        }
    }

    function renderGame() {
        game.render_game('game-canvas', showingSkillTree);
    }

    function renderCharacter(character) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = 'white';
        ctx.font = '16px Arial';
        ctx.fillText(`Name: ${character.name}`, 10, 30);
        ctx.fillText(`Level: ${character.level} (EXP: ${character.experience})`, 10, 50);
        ctx.fillText(`Race: ${character.race}`, 10, 70);
        ctx.fillText(`Profession: ${character.profession}`, 10, 90);
        ctx.fillText(`Gold: ${character.gold}`, 10, 110);
        ctx.fillText(`Skill Points: ${character.skill_points}`, 10, 130);
        ctx.fillText(`Location: ${character.location}`, 10, 150);

        document.getElementById('base-stats').innerHTML = `
            Strength: ${character.strength}<br>
            Endurance: ${character.endurance}<br>
            Wisdom: ${character.wisdom}<br>
            Mystic: ${character.mystic}<br>
            Agility: ${character.agility}<br>
            Accuracy: ${character.accuracy}<br>
            Intellect: ${character.intellect}<br>
            Luck: ${character.luck}
        `;
        document.getElementById('skills').innerHTML = `
            Sword: ${character.sword}<br>
            Spear: ${character.spear}<br>
            Axe: ${character.axe}<br>
            Dagger: ${character.dagger}<br>
            Bow: ${character.bow}<br>
            Shield: ${character.shield_skill}<br>
            Magic: ${character.magic}<br>
            Rune Magic: ${character.rune_magic}<br>
            Magic Resistance: ${character.magic_resistance}<br>
            Healing: ${character.healing}<br>
            Mining: ${character.mining}<br>
            Fishing: ${character.fishing}<br>
            Alchemy: ${character.alchemy}
        `;
        document.getElementById('other-stats').innerHTML = `
            Race: ${character.race}<br>
            Level: ${character.level}<br>
            Experience: ${character.experience}<br>
            Guild: ${character.guild || 'None'}<br>
            Gold: ${character.gold}
        `;
        document.getElementById('equipment').innerHTML = `
            Armor: ${character.equipment.armor || 'None'}<br>
            Helmet: ${character.equipment.helmet || 'None'}<br>
            Amulet: ${character.equipment.amulet || 'None'}<br>
            Gloves: ${character.equipment.gloves || 'None'}<br>
            Ring: ${character.equipment.ring || 'None'}<br>
            Weapon: ${character.equipment.weapon || 'None'}<br>
            Shield: ${character.equipment.shield || 'None'}<br>
            Legs: ${character.equipment.legs || 'None'}<br>
            Boots: ${character.equipment.boots || 'None'}
        `;
    }

    function renderSkillTree() {
        game.render_game('game-canvas', true);
    }

    document.getElementById('connect-wallet').addEventListener('click', async () => {
        const { solana } = window;
        if (solana && solana.isPhantom) {
            try {
                const response = await solana.connect();
                wallet = response.publicKey.toString();
                document.getElementById('wallet').value = wallet;
                connectWebSocket(wallet);
            } catch (err) {
                console.error('Wallet connection failed:', err);
            }
        } else {
            alert('Please install Phantom wallet');
        }
    });

    document.getElementById('create-character').addEventListener('click', async () => {
        const name = document.getElementById('name').value;
        const race = document.getElementById('race').value;
        const profession = document.getElementById('profession').value;

        try {
            game.create_character(name, race, profession, wallet);
            const character = game.get_character();
            renderGame();
            skillsMenu.classList.remove('hidden');
            equipmentWindow.classList.remove('hidden');
            skillTreeControls.classList.remove('hidden');
            itemControls.classList.remove('hidden');
            sendPlayerUpdate(character);
            console.log('Character NFT creation on Solana TBD');
        } catch (e) {
            console.error('Error creating character:', e);
        }
    });

    document.getElementById('explore').addEventListener('click', () => {
        try {
            const result = game.fight_monster();
            combatLog.innerText = result;
            const character = game.get_character();
            if (!showingSkillTree) {
                renderGame();
            } else {
                renderSkillTree();
            }
            sendPlayerUpdate(character);
            console.log('Character NFT update on Solana TBD');
        } catch (e) {
            console.error('Error fighting monster:', e);
        }
    });

    document.getElementById('toggle-skills').addEventListener('click', () => {
        skillsMenu.classList.toggle('hidden');
        equipmentWindow.classList.toggle('hidden');
        if (!showingSkillTree) {
            renderGame();
        }
    });

    document.getElementById('toggle-skill-tree').addEventListener('click', () => {
        showingSkillTree = !showingSkillTree;
        if (showingSkillTree) {
            renderSkillTree();
        } else {
            renderGame();
        }
    });

    document.getElementById('unlock-node').addEventListener('click', () => {
        const nodeId = parseInt(document.getElementById('node-id').value);
        try {
            game.unlock_skill_node(nodeId);
            const character = game.get_character();
            if (showingSkillTree) {
                renderSkillTree();
            } else {
                renderGame();
            }
            sendPlayerUpdate(character);
            console.log('Character NFT update on Solana TBD');
        } catch (e) {
            combatLog.innerText = `Error: ${e}`;
        }
    });

    document.getElementById('equip-item').addEventListener('click', () => {
        const itemId = parseInt(document.getElementById('item-id').value);
        try {
            game.equip_item(itemId);
            const character = game.get_character();
            renderGame();
            sendPlayerUpdate(character);
            console.log('Character NFT update on Solana TBD');
        } catch (e) {
            combatLog.innerText = `Error: ${e}`;
        }
    });

    function updatePosition(dx, dy) {
        const character = game.get_character();
        const newX = character.x + dx;
        const newY = character.y + dy;
        const location = document.getElementById('location').value;
        try {
            game.update_position(newX, newY, location);
            renderGame();
            sendPlayerUpdate(character);
            console.log('Character NFT update on Solana TBD');
        } catch (e) {
            combatLog.innerText = `Error: ${e}`;
        }
    }

    document.getElementById('move-up').addEventListener('click', () => updatePosition(0, -1));
    document.getElementById('move-down').addEventListener('click', () => updatePosition(0, 1));
    document.getElementById('move-left').addEventListener('click', () => updatePosition(-1, 0));
    document.getElementById('move-right').addEventListener('click', () => updatePosition(1, 0));

    document.getElementById('location').addEventListener('change', async () => {
        const character = game.get_character();
        const location = document.getElementById('location').value;
        try {
            await loadMap(location);
            game.update_position(character.x, character.y, location);
            renderGame();
            sendPlayerUpdate(character);
            console.log('Character NFT update on Solana TBD');
        } catch (e) {
            combatLog.innerText = `Error: ${e}`;
        }
    });

    canvas.addEventListener('click', (event) => {
        const rect = canvas.getBoundingClientRect();
        const mx = event.clientX - rect.left;
        const my = event.clientY - rect.top;
        const character = game.get_character();
        const players = game.other_players;
        const tileSize = 32;
        const tileX = Math.floor((mx - canvas.width / 2) / tileSize + (my - canvas.height / 4) / (tileSize / 2));
        const tileY = Math.floor((my - canvas.height / 4) / (tileSize / 2) - (mx - canvas.width / 2) / tileSize);

        for (const player of players) {
            if (Math.abs(player.x - tileX) < 1 && Math.abs(player.y - tileY) < 1 && player.location === character.location) {
                playerInfo.classList.remove('hidden');
                document.getElementById('player-details').innerHTML = `
                    Name: ${player.name}<br>
                    Level: ${player.level}<br>
                    Armor: ${player.equipment.armor || 'None'}<br>
                    Helmet: ${player.equipment.helmet || 'None'}<br>
                    Amulet: ${player.equipment.amulet || 'None'}<br>
                    Gloves: ${player.equipment.gloves || 'None'}<br>
                    Ring: ${player.equipment.ring || 'None'}<br>
                    Weapon: ${player.equipment.weapon || 'None'}<br>
                    Shield: ${player.equipment.shield || 'None'}<br>
                    Legs: ${player.equipment.legs || 'None'}<br>
                    Boots: ${player.equipment.boots || 'None'}
                `;
                return;
            }
        }
    });

    canvas.addEventListener('contextmenu', (event) => {
        event.preventDefault();
        const rect = canvas.getBoundingClientRect();
        const mx = event.clientX - rect.left;
        const my = event.clientY - rect.top;
        const character = game.get_character();
        const players = game.other_players;
        const tileSize = 32;
        const tileX = Math.floor((mx - canvas.width / 2) / tileSize + (my - canvas.height / 4) / (tileSize / 2));
        const tileY = Math.floor((my - canvas.height / 4) / (tileSize / 2) - (mx - canvas.width / 2) / tileSize);

        for (const player of players) {
            if (Math.abs(player.x - tileX) < 1 && Math.abs(player.y - tileY) < 1 && player.location === character.location) {
                try {
                    const result = game.fight_player(player.wallet);
                    combatLog.innerText = result;
                    const updatedCharacter = game.get_character();
                    sendPlayerUpdate(updatedCharacter);
                    renderGame();
                    console.log('Character NFT update on Solana TBD');
                } catch (e) {
                    combatLog.innerText = `Error: ${e}`;
                }
                return;
            }
        }
    });
}

async function connectWallet() {
    const { solana } = window;
    if (solana && solana.isPhantom) {
        try {
            const response = await solana.connect();
            document.getElementById('wallet').value = response.publicKey.toString();
        } catch (err) {
            console.error('Wallet connection failed:', err);
        }
    } else {
        alert('Please install Phantom wallet');
    }
}

document.getElementById('connect-wallet').addEventListener('click', connectWallet);
initGame();
