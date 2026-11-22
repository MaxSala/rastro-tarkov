<p align="center">
  <a href="https://maxsala.github.io/rastro-tarkov/pt-br/docs/guide/">Site</a> · <a href="../../README.pt-BR.md">Voltar ao README</a> · <a href="FEATURES.md">Recursos</a> · <b>Guia</b> · <a href="FAQ.md">Perguntas e segurança</a>
</p>

# Guia

## Instalação

1. **[Baixe o `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Este link sempre traz a versão mais recente.
2. Coloque onde quiser: na área de trabalho, numa pasta em Documentos, em qualquer lugar. Ele não precisa de nenhum outro arquivo ao lado.
3. Dê um clique duplo. Se o Windows mostrar um aviso azul, veja [É seguro?](FAQ.md#é-seguro)

Requer Windows 10 ou 11, 64 bits.

### Primeira execução

Na primeira vez, abre um guia curto:

1. **Pasta de prints.** O Rastro encontra `Documents\Escape from Tarkov\Screenshots` sozinho, mesmo quando o Windows moveu Documentos para o OneDrive. Se não encontrar, cole o caminho e ele mostra quantos prints com coordenadas está vendo. Aqui você também pode ativar a detecção de raid.
2. **Mapas.** Baixe os mapas da comunidade do tarkov.dev (recomendado), importe um pacote de mapas ou experimente o mapa de demonstração.
3. Pronto. Tire um print no jogo e sua seta aparece no mapa.

Para usar o Mercado, a tabela de Munição e os nomes das chaves, abra a aba Mercado uma vez e clique em *Baixar preços*.

### Atualizando

Se você permitiu a busca de atualizações, quando sair uma versão nova aparece um botão na barra superior que leva à página de download. Se não, baixe o novo `.exe` pelo [link acima](#instalação), feche o Rastro e use o arquivo novo no lugar do antigo. Suas configurações, pontos, mapas e preços ficam [separados](#onde-ficam-seus-dados), então nada se perde.

Depois de atualizar, o Rastro mostra o que mudou. O histórico completo está em *Configurações → Sobre → Novidades* e no [CHANGELOG.md](../../CHANGELOG.md). Se uma atualização trouxer dados de mapa novos, use *Atualizar pelo tarkov.dev* no painel de Mapas para pegá-los.

### Atalhos

| Ação | Como |
|---|---|
| Atualizar sua posição | Tirar um print no jogo |
| Trocar de seção (Mapa, Mercado, Munição, Missões, Esconderijo) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
| Paleta de comandos | `Ctrl+K` |
| Painéis de extrações, pontos, grupo, mapas e missões | `1`, `2`, `3`, `4`, `5` |
| Nova raid (limpar rastro e marcações) | `N` |
| Ver o mapa inteiro / seguir sua posição | `F` / `G` |
| Zoom | `+` / `-` ou a roda do mouse |
| Adicionar um ponto, guiar até um lugar | Botão direito no mapa |
| Pingar para o squad | `Shift` + clique |
| Buscar no Mercado | `/` |
| Configurações | `,` |

### Onde ficam seus dados

As configurações, seus pontos, seu progresso de missões, os mapas baixados e os caches de preços e missões ficam em uma pasta, separada do `.exe`:

| Sistema | Pasta |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

Em *Configurações → Registro* há um botão para abri-la.

### Se algo não funcionar

- **Abre no navegador em vez da janela própria.** O Rastro usa o Microsoft WebView2 para desenhar a janela. Ele já vem no Windows 10 e 11, mas se tiver sido removido, o Rastro abre no seu navegador. Instalar o [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) traz a janela de volta.
- **Uma página de erro diz que a porta já está em uso.** Provavelmente outra cópia do Rastro ainda está aberta. Feche pela barra de tarefas ou pelo Gerenciador de Tarefas (procure `rastro.exe`) e abra de novo.
- **Minha posição nunca se move.** Confira se a pasta de prints em *Configurações → Geral* é a mesma em que o jogo realmente salva e se você está apertando a tecla de print do jogo.
- **Meus companheiros não conseguem entrar na minha sala.** Eles precisam alcançar o seu PC. Use o endereço da sua VPN mesh (Tailscale, ZeroTier) ao abrir a sala, ou abra a porta do grupo no seu roteador.
- **Ainda travado?** Pergunte no [Discord](https://discord.gg/H4pRxGBGDb). Em *Configurações → Registro* há um botão *Copiar*, e colar esse registro ajuda muito.

## Como funciona

O Rastro lê dois tipos de arquivo que o Escape from Tarkov já salva no seu PC. Ele não precisa de mais nada.

| O que ele lê | Onde | O que o Rastro tira disso |
|---|---|---|
| **Os nomes dos seus prints** | `Documents\Escape from Tarkov\Screenshots` | Sua posição e para onde você está olhando. O Tarkov escreve suas coordenadas no nome de cada print que salva. O Rastro lê o nome, nunca a imagem, e nunca move, edita ou apaga seus prints. |
| **Os logs do jogo** (opcional, desligado até você ativar) | `Battlestate Games\Escape from Tarkov\Logs` | Qual mapa está carregando, quando uma raid começa e se você está jogando PvP ou PvE. Com isso o Rastro troca sozinho para o mapa certo, começa um rastro novo a cada raid e mostra quanto tempo de raid falta. Só o nome do mapa e o modo de jogo são guardados. Também lê as mensagens dos comerciantes, para marcar as missões que você aceita, entrega ou falha. |

Na prática: você aperta a tecla de print no jogo e, um segundo depois, sua seta se move no mapa.

O que o Rastro **não** faz: não lê a memória do jogo, não injeta nada, não se conecta ao processo do jogo, não captura tráfego de rede, não desenha por cima do jogo e não aperta teclas por você. Ele roda como um programa separado e só olha arquivos no disco.

## Compilando a partir do código

Você só precisa disso se quiser mudar o código ou compilar o `.exe` por conta própria. Você vai precisar do [Rust](https://rustup.rs/) 1.85 ou mais novo e do [Node.js](https://nodejs.org/) LTS (no Linux, também `libwebkit2gtk-4.1-dev` e `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat no Windows
```

O resultado é um único executável em `target/release/`. A configuração para desenvolver, a estrutura do projeto, as regras de código e como adicionar uma tradução estão no **[CONTRIBUTING.md](../../CONTRIBUTING.md)** (em inglês).
