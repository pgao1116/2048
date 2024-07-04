const fetchGameState = async () => {
    try {
        const resp = await fetch("http://localhost:8000/gamestate");
        if (!resp.ok) {
            throw new Error(`http status: ${resp.status}`);
        }
        return await resp.json();
    } catch (err) {
        console.error("couldn't fetch state:", err); 
        return null;
    }
};

const sendKeyStroke = async (keystroke) => {    
    try {    
        const resp = await fetch("http://localhost:8000/keystroke", {
            method: "POST", 
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
            },
            body: JSON.stringify({ [keystroke]: null }),     
        });

        if (!resp.ok) { throw new Error(`http status: ${resp.status}`); }
        return await resp.json(); 
    
    } catch (err) {
        console.error("couldn't send keystroke:", err); 
    }
};

const restartGame = async () => {
    try {
        const gameState = await sendKeyStroke('RestartGame');
        if (gameState) {
            document.getElementById("score").textContent = gameState.score;
            document.getElementById("best").textContent = gameState.best;
            updateGrid(gameState.mat);
        } else {
            console.error("Failed to fetch game state after restart");
        }
    } catch (err) {
        console.error("couldn't restart game:", err); 
    }
};

const updateGrid = (grid) => {
    grid.forEach((row, rowIndex) => {
        row.forEach((value, colIndex) => {
            const block = document.getElementById(`block${rowIndex * 4 + colIndex + 1}`);
            block.textContent = value === 0 ? '' : value;
            block.dataset.value = value;
        });
    });
};

document.addEventListener("DOMContentLoaded", async function () {
    try {
        const gameState = await fetchGameState();
        if (gameState) {
            document.getElementById("score").textContent = gameState.score;
            document.getElementById("best").textContent = gameState.best;
            updateGrid(gameState.mat);
        } else {
            console.error("Failed to fetch game state");
        }
    } catch (error) {
        console.error("Error in DOMContentLoaded event listener:", error);
    }
});


// Main loop...
document.addEventListener("keydown", async function (event) {
    const keyMap = {
        ArrowUp: 'KeyUp',
        ArrowDown: 'KeyDown',
        ArrowLeft: 'KeyLeft',
        ArrowRight: 'KeyRight',
        r: 'RestartGame' // Added Restart Game key
    };

    if (keyMap[event.key]) {
        if (keyMap[event.key] === 'RestartGame') {
            await restartGame();
        } else {
            await sendKeyStroke(keyMap[event.key]);
        }
        const gameState = await fetchGameState();
        if (gameState) {
            document.getElementById("score").textContent = gameState.score;
            document.getElementById("best").textContent = gameState.best;
            updateGrid(gameState.mat);
        }
    }
});

// Add event listener for the Restart Game button
document.querySelector(".restart").addEventListener("click", async () => {
    await restartGame();
});

