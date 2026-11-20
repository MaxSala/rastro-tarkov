<p align="center">
  <img src="docs/images/banner.png" alt="Rastro: um mapa de raid ao vivo para Escape from Tarkov, no seu segundo monitor" width="100%">
</p>

<p align="center">
  🌐 <a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ru.md">Русский</a> · <a href="README.de.md">Deutsch</a> · <b>Português</b> · <a href="README.fr.md">Français</a> · <a href="README.pl.md">Polski</a> · <a href="README.zh-CN.md">中文</a>
  <br>
  <sub>Tradução da comunidade. Se algo for diferente do <a href="README.md">README em inglês</a>, vale a versão em inglês. Achou um erro? <a href="https://discord.gg/H4pRxGBGDb">Avise a gente no Discord</a>.</sub>
</p>

<p align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest"><img src="https://img.shields.io/github/v/release/MaxSala/rastro-tarkov?label=%C3%9Altima%20vers%C3%A3o&color=f0a43a" alt="Última versão"></a>
  <img src="https://img.shields.io/badge/Status-Beta%20p%C3%BAblico-f0a43a" alt="Beta público">
  <a href="https://github.com/MaxSala/rastro-tarkov/stargazers"><img src="https://img.shields.io/github/stars/MaxSala/rastro-tarkov?label=Estrelas&color=f0a43a&logo=github" alt="Estrelas no GitHub"></a>
  <a href="https://discord.gg/H4pRxGBGDb"><img src="https://img.shields.io/badge/Discord-Entre%20na%20comunidade-5865F2?logo=discord&logoColor=white" alt="Entre no Discord"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows&logoColor=white" alt="Windows 10 e 11">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/MaxSala/rastro-tarkov?color=4c4c4c" alt="Licença MIT"></a>
</p>

<h3 align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe">⬇ Baixar o Rastro para Windows</a>
</h3>

<p align="center">
  Grátis e de código aberto · Sem instalador · Sem conta · Nada para configurar
  <br>
  <a href="#como-funciona">Como funciona</a> ·
  <a href="#jogue-com-seu-squad">Modo squad</a> ·
  <a href="#é-seguro">É seguro?</a> ·
  <a href="#instalação">Instalação</a> ·
  <a href="#para-onde-o-rastro-está-indo">Roadmap</a> ·
  <a href="#participe">Participe</a> ·
  <a href="https://discord.gg/H4pRxGBGDb">Discord</a>
</p>

> [!TIP]
> **O Rastro está em beta público, e isso é só o começo.** As missões acabaram de chegar; o esconderijo, missões em grupo e muito mais estão a caminho, e a comunidade decide o que vem primeiro.
> Se você curte o rumo do projeto, **[⭐ dê uma estrela no repositório](https://github.com/MaxSala/rastro-tarkov/stargazers)** e **[entre no Discord](https://discord.gg/H4pRxGBGDb)**. Leva cinco segundos e ajuda mais do que você imagina.

---

**O Rastro é um mapa ao vivo da sua raid que roda no seu segundo monitor.** Tire um print no jogo e sua posição aparece no mapa, com a distância e a direção até cada extração. Abra uma sala e seu squad se vê no mesmo mapa, cada um no seu PC. Passe o mouse sobre uma porta trancada para ver qual chave a abre e quanto ela custa. Acompanhe suas missões com cada objetivo pendente no mapa. Confira preços e munições sem dar alt-tab para o navegador.

Ele nunca mexe no jogo. O Rastro só lê arquivos que o Tarkov já grava no seu disco, e é grátis, de código aberto e feito por jogadores para jogadores.

## Capturas de tela

<table>
  <tr>
    <td width="50%"><img src="docs/images/party.png" alt="Três membros do squad em Customs, com o cartão de um companheiro"></td>
    <td width="50%"><img src="docs/images/map.png" alt="Customs com sua posição, seu rastro e a rota até uma extração"></td>
  </tr>
  <tr>
    <td align="center"><b>Squad.</b> A posição e a direção de todos no mesmo mapa, com pings. Sem servidor no meio.</td>
    <td align="center"><b>Mapa.</b> Sua posição e seu rastro, todas as extrações e distância e direção ao vivo até a que você escolher.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/floors.png" alt="Os dormitórios no segundo andar, com o resto do mapa e os companheiros de baixo escurecidos"></td>
    <td width="50%"><img src="docs/images/keys.png" alt="A camada de fechaduras com o cartão de uma chave aberto"></td>
  </tr>
  <tr>
    <td align="center"><b>Andares.</b> Segue você de um andar para outro e escurece tudo o que não está no seu.</td>
    <td align="center"><b>Chaves.</b> Cada porta trancada do mapa, qual chave a abre e quanto ela custa.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/market.png" alt="Busca no mercado com as ofertas de um item"></td>
    <td width="50%"><img src="docs/images/ammo.png" alt="Tabela de munição de um calibre"></td>
  </tr>
  <tr>
    <td align="center"><b>Mercado.</b> Busca instantânea e offline com preços do flea e dos comerciantes, ordenados por valor por slot.</td>
    <td align="center"><b>Munição.</b> Dano, penetração e dano à armadura de cada munição, com o lugar mais barato para comprar.</td>
  </tr>
</table>

<sub>Mapas dos <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">colaboradores do tarkov.dev</a>, CC BY-NC-SA 4.0. Preços e dados de itens do <a href="https://tarkov.dev">tarkov.dev</a>. As capturas do squad usam posições de exemplo. As capturas mostram o app em inglês.</sub>

## Para onde o Rastro está indo

O Rastro nasceu como um mapa no segundo monitor para você nunca mais se perder. O plano é bem maior: **um único app de apoio para tudo o que você consulta entre as raids e durante elas**, sempre sem mexer no jogo. Isto é o que está na mesa:

| | O que vem aí | O que muda para você |
|---|---|---|
| 👥 | **Missões do squad** | As missões pendentes dos seus companheiros no mesmo mapa, para o squad planejar uma rota só. |
| 🏠 | **Rastreador do esconderijo** | Cada melhoria, o que ela precisa e quais itens guardar. Chega de vender hoje o que você vai precisar amanhã. |
| 🔑 | **Seu chaveiro** | Marque as chaves que você tem e as portas que você pode abrir ganham destaque em todos os mapas. |
| 🛒 | **Lista de compras** | Favorite itens e receba um aviso quando o preço deles mudar. |
| 📊 | **Histórico de raids** | Mapa, duração, tempo de fila e modo de jogo de cada raid. |
| 🎯 | **Munição contra armadura** | Um gráfico de penetração de cada munição contra cada classe de armadura. |
| 📍 | **Compartilhe seus pontos** | Mande seus pontos de loot e esconderijos para o squad, ou leve-os para outro PC. |

Nada disso tem data ainda, e a ordem não está fechada: **ela é definida pelo que os jogadores pedem.** A lista completa, com itens menores e o que já foi lançado, está em [docs/ROADMAP.md](docs/ROADMAP.md) (em inglês).

**Tem uma ideia?** Poste no fórum de feature requests do [Discord](https://discord.gg/H4pRxGBGDb) ou [abra uma issue](https://github.com/MaxSala/rastro-tarkov/issues). Toda sugestão é lida, e as mais pedidas sobem na fila.

## Como funciona

O Rastro lê dois tipos de arquivo que o Escape from Tarkov já salva no seu PC. Ele não precisa de mais nada.

| O que ele lê | Onde | O que o Rastro tira disso |
|---|---|---|
| **Os nomes dos seus prints** | `Documents\Escape from Tarkov\Screenshots` | Sua posição e para onde você está olhando. O Tarkov escreve suas coordenadas no nome de cada print que salva. O Rastro lê o nome, nunca a imagem, e nunca move, edita ou apaga seus prints. |
| **Os logs do jogo** (opcional, desligado até você ativar) | `Battlestate Games\Escape from Tarkov\Logs` | Qual mapa está carregando, quando uma raid começa e se você está jogando PvP ou PvE. Com isso o Rastro troca sozinho para o mapa certo, começa um rastro novo a cada raid e mostra quanto tempo de raid falta. Só o nome do mapa e o modo de jogo são guardados. Também lê as mensagens dos comerciantes, para marcar as missões que você aceita, entrega ou falha. |

Na prática: você aperta a tecla de print no jogo e, um segundo depois, sua seta se move no mapa.

O que o Rastro **não** faz: não lê a memória do jogo, não injeta nada, não se conecta ao processo do jogo, não captura tráfego de rede, não desenha por cima do jogo e não aperta teclas por você. Ele roda como um programa separado e só olha arquivos no disco.

## Jogue com seu squad

O modo grupo do Rastro coloca todo o seu squad no mesmo mapa, cada um com seu nome e sua cor.

- **Veja onde está cada um.** A seta de cada companheiro mostra onde ele estava e para onde olhava no último print. Se alguém estiver em outro andar, a seta fica mais apagada e, passando o mouse, você vê em qual andar ele está.
- **Marque um lugar.** `Shift` + clique em qualquer ponto do mapa e o lugar pisca na tela de todo mundo.
- **Fácil de configurar.** Um jogador abre uma sala e copia um código de convite. Os outros colam em *Entrar*. Pronto. Até 8 jogadores.
- **Sem servidor, sem conta.** As posições vão direto de PC para PC. Ninguém mais vê, nem a gente.
- **Criptografado.** Todo o tráfego tem criptografia de ponta a ponta com o [protocolo Noise](https://noiseprotocol.org/). O código de convite é a chave, então compartilhe só com o seu squad.

Os outros precisam conseguir alcançar o PC do anfitrião. O jeito mais fácil é uma VPN mesh gratuita como [Tailscale](https://tailscale.com/) ou [ZeroTier](https://www.zerotier.com/): cada um instala uma vez e o Rastro funciona como se vocês estivessem na mesma rede, sem abrir portas no roteador. Abrir uma porta no roteador do anfitrião também funciona.

Cada jogador continua usando o Rastro no próprio PC e tirando os próprios prints: é isso que mantém a posição de todos atualizada.

## Recursos

**Mapa**
- Sua posição e direção, atualizadas a cada print, com um rastro que vai sumindo por onde você passou.
- Clique em uma extração ou em qualquer ponto para ver a distância, para que lado virar e a diferença de altura.
- Extrações filtradas por PMC, Scav ou compartilhadas. Marque as que o jogo diz que estão abertas nesta raid e o resto fica apagado.
- Camadas do tarkov.dev: transições, interruptores, contêineres de loot (com filtro por tipo) e fechaduras.
- Mapas com vários andares seguem você automaticamente, ou você escolhe um no painel de andares. O resto do mapa e tudo o que está em outros andares fica escurecido.
- Passe o mouse sobre qualquer coisa para ver um cartão com o que é, a distância, em que andar está e o que acontece ao clicar.
- Seus próprios pontos: clique com o botão direito no mapa para salvar pontos de loot, áreas perigosas, esconderijos ou locais de missão. Eles ficam salvos em coordenadas do jogo, então sobrevivem à reinstalação de um mapa.

**Missões**
- Todas as missões no seu idioma: as que você está fazendo, as que pode começar agora e, para o resto, exatamente o que falta (seu nível ou as missões anteriores). Filtre por comerciante, mapa ou Kappa, ou busque missões, objetivos e itens.
- **As missões seguem o jogo.** Com a detecção de raid ligada, as missões que você aceita, entrega ou falha se marcam sozinhas pelos logs do jogo, PvP e PvE separados. Na primeira vez, o Rastro lê todas as sessões que o jogo guardou, então no meio do wipe você já fica em dia. Sem logs? *Atualize o progresso* escolhendo a última missão feita com cada comerciante.
- Cada missão mostra o comerciante, a imagem dela e as recompensas: experiência, reputação, itens e as ofertas e fabricações que ela desbloqueia.
- Seus objetivos pendentes no mapa, honestos com os dados: um selo para um lugar exato, o contorno real para uma área e pontinhos para os lugares onde um item *pode* aparecer. Clique e o Rastro te guia, como até uma extração. Viu um errado? *Local errado?* avisa com um clique.
- **Plano da raid** no painel *Missões*: marque o que vai fazer nesta raid e o mapa mostra só isso, numerado pela rota mais curta, com o que levar. *Próximo* mostra o próximo objetivo em destaque; *Para entregar*, o que já está pronto.
- Missões por comerciante, como no jogo, com seu progresso de Kappa e Lightkeeper, o mapa onde você mais avança e *Itens*: o que suas missões ainda pedem, para você não vender.

**Chaves**
- A camada *Fechaduras* mostra cada porta, contêiner e porta-malas que precisa de chave. As que precisam de energia têm contorno amarelo.
- Clique em uma fechadura para ver qual chave a abre, onde comprar mais barato e o melhor preço de venda.
- *Destacar esta chave* escurece todas as outras fechaduras, para você ver tudo o que uma única chave abre.
- Cada chave no Mercado mostra em quais mapas instalados ela é usada, com um link que leva direto até lá.

**Mercado**
- Busque no catálogo completo de itens pelo nome ou nome curto. Acentos e ordem das palavras não importam. Aperte `/` para buscar e as setas para navegar pelos resultados.
- Ordene por melhor venda, preço no flea, média de 24 horas, variação de 48 horas ou preço por slot do inventário.
- Cada item lista todas as ofertas de comerciantes e do flea, com o nível de comerciante necessário e o preço original em dólares ou euros.
- O catálogo é baixado uma vez e pesquisado no seu PC, então as buscas são instantâneas e funcionam offline.

**Munição**
- Todas as munições agrupadas por calibre e ordenadas por penetração.
- Dano (por chumbo × número de chumbos nas de chumbo grosso), penetração, dano à armadura, chance de fragmentação, velocidade na boca do cano e o lugar mais barato para comprar. Traçantes são marcadas.

**Detecção de raid** (opcional)
- Troca para o mapa certo quando uma raid carrega, limpa seu rastro e suas marcações no início de cada raid e mostra o tempo restante. Lê os logs do jogo, como explicado em [Como funciona](#como-funciona).

**Todo o resto**
- 16 idiomas, os mesmos que o tarkov.dev suporta: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe e 中文. Nomes de itens, mapas e extrações também seguem o seu idioma. Se você mudar, o Rastro oferece passar seus mapas, preços e missões para o novo idioma.
- Dados de PvP e PvE, escolhidos automaticamente pelos logs do jogo ou manualmente.
- Avisos de atualização opcionais e uma página de *Novidades* depois de cada atualização.
- Atalhos de teclado para tudo e uma paleta de comandos (`Ctrl+K`).

## É seguro?

O Rastro é um programa pequeno e portátil. Veja o que isso significa e como você pode conferir.

- **Sem instalador e sem permissão de administrador.** É um único `.exe`. Não instala serviços nem drivers, não inicia com o Windows e não mexe nos arquivos do jogo. As configurações e os mapas baixados ficam em uma pasta (veja [Onde ficam seus dados](#onde-ficam-seus-dados)), e apagando essa pasta e o `.exe` ele some por completo.
- **Código aberto.** Todo o código está neste repositório. Qualquer pessoa pode ler.
- **Compilado em público.** Cada versão é compilada pelos próprios servidores do GitHub direto do código-fonte, não no PC de alguém. Você vê cada compilação na aba [Actions](https://github.com/MaxSala/rastro-tarkov/actions), e a página de cada versão mostra o SHA-256 do arquivo para você conferir que baixou exatamente ele:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Não manda nada para fora.** Sem telemetria, sem anúncios, sem conta. Ele só se conecta à internet quando você pede, e a lista completa de conexões está em [Privacidade](#privacidade-e-uso-de-rede).

**"O Windows protegeu o computador".** Na primeira vez que você abrir o Rastro, o Windows SmartScreen provavelmente vai mostrar uma janela azul dizendo que impediu um aplicativo não reconhecido. Isso acontece com qualquer programa novo sem um certificado pago, não porque o arquivo tenha algum problema. Clique em **Mais informações** e depois em **Executar assim mesmo**. O Windows lembra da sua escolha. Alguns antivírus também desconfiam de programas novos sem assinatura. Se o seu marcar o Rastro, você pode restaurá-lo e adicionar uma exceção, enviar o arquivo ao [VirusTotal](https://www.virustotal.com/) para uma segunda opinião e reportar um falso positivo ao fabricante do antivírus.

> [!IMPORTANT]
> O Rastro é um projeto de fãs não oficial, sem vínculo com a Battlestate Games ou o tarkov.dev e sem o apoio deles. Ele foi feito para ficar fora do jogo: só lê arquivos no disco e nunca interage com o processo do jogo. Mesmo assim, usar ferramentas de terceiros com o Escape from Tarkov é responsabilidade sua, de acordo com os termos de serviço da Battlestate Games.

## Instalação

1. **[Baixe o `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Este link sempre traz a versão mais recente.
2. Coloque onde quiser: na área de trabalho, numa pasta em Documentos, em qualquer lugar. Ele não precisa de nenhum outro arquivo ao lado.
3. Dê um clique duplo. Se o Windows mostrar um aviso azul, veja [É seguro?](#é-seguro)

Requer Windows 10 ou 11, 64 bits.

### Primeira execução

Na primeira vez, abre um guia curto:

1. **Pasta de prints.** O Rastro encontra `Documents\Escape from Tarkov\Screenshots` sozinho, mesmo quando o Windows moveu Documentos para o OneDrive. Se não encontrar, cole o caminho e ele mostra quantos prints com coordenadas está vendo. Aqui você também pode ativar a detecção de raid.
2. **Mapas.** Baixe os mapas da comunidade do tarkov.dev (recomendado), importe um pacote de mapas ou experimente o mapa de demonstração.
3. Pronto. Tire um print no jogo e sua seta aparece no mapa.

Para usar o Mercado, a tabela de Munição e os nomes das chaves, abra a aba Mercado uma vez e clique em *Baixar preços*.

### Atualizando

Se você permitiu a busca de atualizações, quando sair uma versão nova aparece um botão na barra superior que leva à página de download. Se não, baixe o novo `.exe` pelo [link acima](#instalação), feche o Rastro e use o arquivo novo no lugar do antigo. Suas configurações, pontos, mapas e preços ficam [separados](#onde-ficam-seus-dados), então nada se perde.

Depois de atualizar, o Rastro mostra o que mudou. O histórico completo está em *Configurações → Sobre → Novidades* e no [CHANGELOG.md](CHANGELOG.md). Se uma atualização trouxer dados de mapa novos, use *Atualizar pelo tarkov.dev* no painel de Mapas para pegá-los.

### Atalhos

| Ação | Como |
|---|---|
| Atualizar sua posição | Tirar um print no jogo |
| Trocar de seção (Mapa, Mercado, Munição, Missões) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4` |
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

## Perguntas

**Posso ser banido por usar?**
O Rastro nunca lê a memória do jogo, nunca mexe no processo do jogo e nunca envia comandos para o jogo. Ele lê nomes de prints e arquivos de log depois que o jogo os escreveu, os mesmos arquivos que você pode abrir no Explorador. Mas não podemos falar pela Battlestate Games, então veja a nota em [É seguro?](#é-seguro)

**Funciona no PvE?**
Sim. A detecção de raid foi testada com logs de PvE, e o Rastro baixa os preços e dados de PvE quando detecta PvE.

**Precisa de internet?**
Só para baixar mapas e preços, buscar atualizações (se você permitir) e jogar em squad. Todo o resto funciona offline.

**Por que prints?**
São o único lugar onde o jogo grava sua posição exata no disco. Alguns segundos de atraso em troca de nunca mexer no jogo.

**Linux ou macOS?**
O Rastro compila e roda no Linux; o macOS não foi testado. O próprio Tarkov só roda no Windows, então o download é só para Windows. Veja [Compilando a partir do código](#compilando-a-partir-do-código).

## Privacidade e uso de rede

O Rastro não tem conta, telemetria nem anúncios. A interface só conversa com o próprio app em `127.0.0.1`, protegida por uma chave aleatória que muda a cada execução. Tudo o que ele lê do seu disco fica no seu PC. O resto só acontece quando você pede:

| Quando | Conecta a |
|---|---|
| Sempre | Nada fora do seu PC |
| Você baixa mapas do tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` e, como alternativa, `api.tarkov.dev` |
| Você baixa ou atualiza preços | `json.tarkov.dev`, mais `assets.tarkov.dev` para os ícones dos itens |
| Você baixa ou atualiza missões | `json.tarkov.dev` e, como alternativa, `api.tarkov.dev`, mais `assets.tarkov.dev` para as imagens |
| Você importa um pacote de mapas de uma URL | Só o endereço que você digitar (só HTTPS, nunca endereços internos) |
| Você abre uma sala ou entra em um grupo | Direto aos PCs dos seus companheiros |
| Uma vez por execução, se você aceitou a busca de atualizações (ou ao clicar em *Verificar agora*) | `api.github.com`, para ler o número da última versão. Nada sobre você é enviado. |

Cada servidor de download está em uma lista fixa de permitidos, verificada também nos redirecionamentos, e tudo o que é baixado é validado antes do uso. O modelo de ameaças completo está em [docs/SECURITY.md](docs/SECURITY.md) (em inglês).

## Participe

O Rastro é feito de forma aberta por uma equipe pequena, e queremos crescer junto com quem usa. Se você tem cinco segundos ou cinco horas por semana, existe um jeito de ajudar.

### ⭐ Apoie o projeto

- **Dê uma estrela no repositório.** É de graça, é um clique e é o principal jeito de novos jogadores encontrarem o Rastro no GitHub.
- **Watch → Custom → Releases** para ser avisado quando sair uma versão nova.
- **Conte para o seu squad.** O modo grupo é melhor com amigos, e a maioria das pessoas nos encontra pelo boca a boca.
- **Compartilhe um clipe ou print** do Rastro em ação e marque a gente no Discord.

### 💬 Entre na comunidade

O **[Discord do Rastro](https://discord.gg/H4pRxGBGDb)** é onde tudo acontece: anúncios de versões, o roadmap ao vivo, ajuda de outros jogadores, fóruns para bugs e ideias e um lugar para achar squad para testar o modo grupo. O desenvolvimento acontece lá todos os dias, então é o melhor lugar para ver o que vem por aí antes de todo mundo.

### 🙋 Estamos procurando gente

Temos muito mais ideias do que mãos. Se você se encaixa em algum destes perfis, adoraríamos falar com você:

| Função | O que você faria |
|---|---|
| 🛡️ **Moderação** | Manter o Discord amigável e útil enquanto a comunidade cresce. |
| 🌍 **Tradução** | Revisar um dos 14 idiomas que ainda não foram conferidos por falantes nativos, ou manter o seu atualizado. |
| 🦀 **Desenvolvimento** | Rust no backend, React e TypeScript na interface. De pequenas correções a recursos inteiros. |
| 🧪 **Testes** | Testar versões novas antes de todo mundo, principalmente no PvP, em mapas menos jogados e em PCs diferentes. |
| 🗺️ **Mapas e dados** | Conferir posições, andares e fechaduras com o jogo, e ajudar com mapas que o tarkov.dev ainda não cobre. |
| 🎥 **Criadores de conteúdo** | Streamers e criadores de vídeo que queiram mostrar o Rastro ou trazer o feedback da sua comunidade. |

**Como se candidatar:** abra um ticket *Join the team* no canal de suporte do [Discord](https://discord.gg/H4pRxGBGDb). Desenvolvedores e tradutores também podem começar direto com um pull request: o [CONTRIBUTING.md](CONTRIBUTING.md) (em inglês) explica como.

### 💡 Ideias, feedback e bugs

Um feedback sincero é a coisa mais valiosa que você pode dar a um beta. Conte o que confunde, o que falta e o que você usaria em toda raid:

- **Ideias:** o fórum de feature requests do [Discord](https://discord.gg/H4pRxGBGDb) ou uma [issue no GitHub](https://github.com/MaxSala/rastro-tarkov/issues).
- **Bugs:** o fórum de bug reports do Discord ou uma issue no GitHub. Informe sua versão do Rastro (*Configurações → Sobre*) e o que você estava fazendo. Por favor, nunca publique logs completos do jogo, porque eles contêm o ID da sua conta.
- **Problemas de segurança:** nunca em público. Use o reporte privado do GitHub (aba *Security* → *Report a vulnerability*) ou um ticket privado *Security & privacy* no Discord. Detalhes em [docs/SECURITY.md](docs/SECURITY.md).

## Créditos

O Rastro não existiria sem o **[tarkov.dev](https://tarkov.dev)**, o banco de dados de código aberto de Escape from Tarkov mantido pela comunidade **[The Hideout](https://github.com/the-hideout)**. O tarkov.dev começou como [tarkov-tools](https://github.com/kokarn/tarkov-tools), de **Oskar Risberg ([@kokarn](https://github.com/kokarn))**, e foi revivido e aberto pelos colaboradores do The Hideout. O Rastro usa:

- **Mapas:** os mapas SVG de [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps), dos colaboradores do tarkov.dev, sob a licença [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Os autores não permitem o uso dos mapas em cheats ou em software que dê vantagem injusta, como radares. Os mapas são baixados por você e nunca são distribuídos com o Rastro, e cada mapa instalado mantém seus créditos e sua licença.
- **Dados:** extrações, transições, interruptores, contêineres, fechaduras, itens, preços, missões e traduções da API do tarkov.dev e de seus arquivos de dados.
- **Ícones de marcadores:** do [tarkov-dev](https://github.com/the-hideout/tarkov-dev) (`public/maps/interactive`), licença MIT, © 2019 Oskar Risberg.
- **Os ícones de itens** pertencem à Battlestate Games e ficam apenas em cache local para exibição.

Se o Rastro for útil para você, considere apoiar o tarkov.dev, contribuir com ele ou entrar na comunidade deles.

O Rastro também é construído sobre muitos outros projetos de código aberto, como Rust, Tokio, Axum, Snow, Rustls, Wry, React, Leaflet, Tailwind CSS e a fonte Geist. A lista completa com as licenças está no [NOTICE.md](NOTICE.md).

*Escape from Tarkov* é uma marca registrada da Battlestate Games. O conteúdo e os materiais do jogo são marcas e direitos autorais da Battlestate Games e de seus licenciadores.

## Sem fins comerciais

O Rastro é um projeto feito por hobby para a comunidade. É grátis, não tem versão paga, não mostra anúncios e não coleta dados. Vai continuar assim, por escolha e porque os mapas do tarkov.dev em que ele se baseia são licenciados apenas para uso não comercial.

## Limitações conhecidas

- Sua posição só é atualizada quando você tira um print. É de propósito: é o único lugar onde o jogo grava sua posição.
- Labs não tem mapa SVG no tarkov.dev. O Rastro reconhece o Labs, mas só consegue mostrá-lo se você importar seu próprio pacote de mapas.
- A detecção de raid foi conferida com logs de PvE. No PvP deve funcionar igual, mas ainda não foi confirmado com logs reais.
- As traduções para espanhol e inglês foram revisadas. As outras 14 ainda não foram conferidas por falantes nativos. Correções são muito bem-vindas, aqui ou no [Discord](https://discord.gg/H4pRxGBGDb).
- Depois de trocar o idioma ou o modo de jogo (PvP/PvE), é preciso baixar de novo os mapas e os preços para ver os nomes novos.
- Alguns detalhes técnicos nas mensagens de erro e no registro ficam em inglês, seja qual for o seu idioma.

## Compilando a partir do código

Você só precisa disso se quiser mudar o código ou compilar o `.exe` por conta própria. Você vai precisar do [Rust](https://rustup.rs/) 1.85 ou mais novo e do [Node.js](https://nodejs.org/) LTS (no Linux, também `libwebkit2gtk-4.1-dev` e `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat no Windows
```

O resultado é um único executável em `target/release/`. A configuração para desenvolver, a estrutura do projeto, as regras de código e como adicionar uma tradução estão no **[CONTRIBUTING.md](CONTRIBUTING.md)** (em inglês).

## Licença

O código do Rastro é publicado sob a [licença MIT](LICENSE). Os mapas e dados baixados do tarkov.dev mantêm suas próprias licenças, listadas em [Créditos](#créditos) e no [NOTICE.md](NOTICE.md).

---

<p align="center">
  <b>Se o Rastro te ajudou a achar a saída, <a href="https://github.com/MaxSala/rastro-tarkov/stargazers">deixe uma ⭐</a> e <a href="https://discord.gg/H4pRxGBGDb">venha dar um oi no Discord</a>.</b>
  <br>
  <sub>Feito por jogadores, para jogadores. Nos vemos na extração.</sub>
</p>
