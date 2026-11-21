<p align="center">
  <a href="../../README.pt-BR.md">Voltar ao README</a> · <a href="FEATURES.md">Recursos</a> · <a href="GUIDE.md">Guia</a> · <b>Perguntas e segurança</b>
</p>

# Perguntas e segurança

## É seguro?

O Rastro é um programa pequeno e portátil. Veja o que isso significa e como você pode conferir.

- **Sem instalador e sem permissão de administrador.** É um único `.exe`. Não instala serviços nem drivers, não inicia com o Windows e não mexe nos arquivos do jogo. As configurações e os mapas baixados ficam em uma pasta (veja [Onde ficam seus dados](GUIDE.md#onde-ficam-seus-dados)), e apagando essa pasta e o `.exe` ele some por completo.
- **Código aberto.** Todo o código está neste repositório. Qualquer pessoa pode ler.
- **Compilado em público.** Cada versão é compilada pelos próprios servidores do GitHub direto do código-fonte, não no PC de alguém. Você vê cada compilação na aba [Actions](https://github.com/MaxSala/rastro-tarkov/actions), e a página de cada versão mostra o SHA-256 do arquivo para você conferir que baixou exatamente ele:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Não manda nada para fora.** Sem telemetria, sem anúncios, sem conta. Ele só se conecta à internet quando você pede, e a lista completa de conexões está em [Privacidade](#privacidade-e-uso-de-rede).

**"O Windows protegeu o computador".** Na primeira vez que você abrir o Rastro, o Windows SmartScreen provavelmente vai mostrar uma janela azul dizendo que impediu um aplicativo não reconhecido. Isso acontece com qualquer programa novo sem um certificado pago, não porque o arquivo tenha algum problema. Clique em **Mais informações** e depois em **Executar assim mesmo**. O Windows lembra da sua escolha. Alguns antivírus também desconfiam de programas novos sem assinatura. Se o seu marcar o Rastro, você pode restaurá-lo e adicionar uma exceção, enviar o arquivo ao [VirusTotal](https://www.virustotal.com/) para uma segunda opinião e reportar um falso positivo ao fabricante do antivírus.

> [!IMPORTANT]
> O Rastro é um projeto de fãs não oficial, sem vínculo com a Battlestate Games ou o tarkov.dev e sem o apoio deles. Ele foi feito para ficar fora do jogo: só lê arquivos no disco e nunca interage com o processo do jogo. Mesmo assim, usar ferramentas de terceiros com o Escape from Tarkov é responsabilidade sua, de acordo com os termos de serviço da Battlestate Games.

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
O Rastro compila e roda no Linux; o macOS não foi testado. O próprio Tarkov só roda no Windows, então o download é só para Windows. Veja [Compilando a partir do código](GUIDE.md#compilando-a-partir-do-código).

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

Cada servidor de download está em uma lista fixa de permitidos, verificada também nos redirecionamentos, e tudo o que é baixado é validado antes do uso. O modelo de ameaças completo está em [docs/SECURITY.md](../SECURITY.md) (em inglês).

## Limitações conhecidas

- Sua posição só é atualizada quando você tira um print. É de propósito: é o único lugar onde o jogo grava sua posição.
- Labs não tem mapa SVG no tarkov.dev. O Rastro reconhece o Labs, mas só consegue mostrá-lo se você importar seu próprio pacote de mapas.
- A detecção de raid foi conferida com logs de PvE. No PvP deve funcionar igual, mas ainda não foi confirmado com logs reais.
- As traduções para espanhol e inglês foram revisadas. As outras 14 ainda não foram conferidas por falantes nativos. Correções são muito bem-vindas, aqui ou no [Discord](https://discord.gg/H4pRxGBGDb).
- Depois de trocar o idioma ou o modo de jogo (PvP/PvE), é preciso baixar de novo os mapas e os preços para ver os nomes novos.
- Alguns detalhes técnicos nas mensagens de erro e no registro ficam em inglês, seja qual for o seu idioma.

## Sem fins comerciais

O Rastro é um projeto feito por hobby para a comunidade. É grátis, não tem versão paga, não mostra anúncios e não coleta dados. Vai continuar assim, por escolha e porque os mapas do tarkov.dev em que ele se baseia são licenciados apenas para uso não comercial.
