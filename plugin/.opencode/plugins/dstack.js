// dstack plugin for OpenCode.ai
// Injects dstack skill discovery and session bootstrap

const path = require("path");
const fs = require("fs");

module.exports = {
  name: "dstack",
  version: "0.2.0",

  // Called when plugin is loaded
  init(context) {
    const pluginDir = path.dirname(__dirname);
    const skillsDir = path.join(pluginDir, "skills");

    // Register skills directory with OpenCode
    if (context.registerSkillsPath) {
      context.registerSkillsPath("dstack", skillsDir);
    }

    // Inject bootstrap into session start
    if (context.onSessionStart) {
      context.onSessionStart(() => {
        const bootstrapPath = path.join(
          skillsDir,
          "using-dstack",
          "SKILL.md"
        );
        if (fs.existsSync(bootstrapPath)) {
          const content = fs.readFileSync(bootstrapPath, "utf8");
          return {
            systemMessage: content,
          };
        }
        return {};
      });
    }
  },

  // Tool name mapping from Claude Code conventions to OpenCode
  toolMapping: {
    Bash: "bash",
    Read: "read",
    Edit: "edit",
    Write: "write",
    Glob: "glob",
    Grep: "grep",
    Skill: "skill",
    Agent: "agent",
  },
};
