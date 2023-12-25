#set page(
    margin: 20mm,
    footer: align(center, counter(page).display("－1－")),
    footer-descent: 20mm - 10mm,
)

#set text(
    font: ("Times New Roman", "MS Mincho"),
    fallback: false,
    size: 10pt,
    lang: "ja",
    region: "JP",
)

#set figure.caption(separator: h(1em))

#show heading: it => block({
    set text(weight: "regular", size: 10pt)

    if it.numbering != none {
        counter(heading).display(it.numbering)
        h(1em)
    }
    
    it.body
})

#let rules(group: none, ..rules) = {
    set par(leading: 0.3em)

    rect(width: 100%, if group == none {
        rules.pos().map(rule => $ #rule $).join()
    } else {
        align(horizon, grid(
            columns: (1fr, auto),
            gutter: 1em,
            ..rules.pos().map(pair => ($ #pair.at(0) $, [(#group - #pair.at(1))])).flatten(),
        ))
    })
}

#align(center, {
    text(size: 14pt)[(要変更) 代数的エフェクトとハンドラにおける \ Higher-Order Effectsを扱える言語の実装の試み \ ]
    text(size: 12pt)[
        An Attempt to Implement a Language \ Capable of Treating Higher-Order Effects in Algebraic Effects and Handlers \
        #"今村　洸陽（担当教員：中才　恵太朗）" \
        #"大阪公立大学工業高等専門学校　総合工学システム学科　電子情報コース"
    ]
})

= あらまし
代数的エフェクトとハンドラは副作用を含むプログラムに対する新しい手法として近年注目されている．
代数的エフェクトとハンドラが持つ利点の一つが，副作用を発生させるプログラムの記述を変えることなく副作用の動作を変更することが可能であるということである．
しかし，エフェクトを発生させる計算自体を引数とするHigher-Order Effectsを直接扱うことはできず，利点の一つである高いモジュール性を失ってしまう．
本研究では，Higher-Order Effectsを直接扱い代数的エフェクトと同程度のモジュール性を持ったプログラムを記述できるような言語の実装を試みる．

#columns(2, gutter: 10mm)[
    #set heading(numbering: "1.1")

    = はじめに

    = ラムダ計算 <l>
    項の集合$cal(T)$を @l_T ，項の自由変数$italic("FV")$を @l_FV ，代入$[x |-> t]$を @l_subst ，1ステップ評価$-->$を @l_eval で定義する．
    ただし，$cal(V)$は変数の集合，$x$は$cal(V)$のメタ変数，$t$は$cal(T)$のメタ変数であり，フレッシュ変数$italic("fresh"): 2^cal(V) harpoon.rt cal(V)$は$forall V. italic("fresh")(V) in.not V$を満たす．

    #figure(
        caption: [$cal(T)$の生成規則],
        rules(
            $x in cal(T)$,
            $(t in cal(T)) / (lambda x. t in cal(T))$,
            $(t_1 in cal(T) space.quad t_2 in cal(T)) / (t_1 space t_2 in cal(T))$,
        ),
    ) <l_T>

    #figure(
        caption: [$italic("FV")$の生成規則],
        rules(
            $italic("FV")(x) = {x}$,
            $(italic("FV")(t) = V) / (italic("FV")(lambda x. t) = V \\ {x})$,
            $(italic("FV")(t_1) = V_1 space.quad italic("FV")(t_2) = V_2) / (italic("FV")(t_1 space t_2) = V_1 union V_2)$,
        ),
    ) <l_FV>

    #figure(
        caption: "代入規則",
        rules(
            $[x |-> t]x = t$,
            $(x_1 != x_2) / ([x_1 |-> t]x_2 = x_2)$,
            $(italic("fresh")({x_1} union italic("FV")(t_1) union (italic("FV")(t_2) \\ {x_2})) = x_3 \ [x_1 |-> t_1][x_2 |-> x_3]t_2 = t_3) / ([x_1 |-> t_1](lambda x_2. t_2) = lambda x_3. t_3)$,
            $([x |-> t_0]t_1 = t'_1 space.quad [x |-> t_0]t_2 = t'_2) / ([x |-> t_0](t_1 space t_2) = t'_1 space t'_2)$,
        )
    ) <l_subst>

    #figure(
        caption: "1ステップ評価規則",
        rules(
            $(t_1 --> t'_1) / (t_1 space t_2 --> t'_1 space t_2)$,
            $(forall t'_1. not (t_1 --> t'_1) space.quad t_2 --> t'_2) / (t_1 space t_2 --> t_1 space t'_2)$,
            $(forall t'_2. not (t_2 --> t'_2)) / ((lambda x. t_1) space t_2 --> [x |-> t_2]t_1)$,
        ),
    ) <l_eval>

    = 代数的エフェクト
    $cal(T)$を @le_T ，$italic("FV")$を @le_FV ，$[x |-> t]$を @le_subst で拡張し，$-->$を @le_eval で再定義する．
    ただし，$cal(E)$はエフェクトの集合，$e$は$cal(E)$のメタ変数，$x_u$は全体で一意な変数である．

    #figure(
        caption: [$cal(T)$の生成規則の拡張],
        rules(
            $(t in cal(T)) / (e angle.l t angle.r in cal(T))$,
            $(t_1 in cal(T) space.quad t_2 in cal(T)) / (e angle.l t_1 angle.r >>> t_2 in cal(T))$,
            $(t_1 in cal(T) space.quad t_2 in cal(T)) / (e ~> t_1 gt.tri t_2 in cal(T))$,
        ),
    ) <le_T>

    #figure(
        caption: [$italic("FV")$の生成規則の拡張],
        rules(
            $(italic("FV")(t) = V) / (italic("FV")(e angle.l t angle.r) = V)$,
            $(italic("FV")(t_1) = V_1 space.quad italic("FV")(t_2) = V_2) / (italic("FV")(e angle.l t_1 angle.r >>> t_2) = V_1 union V_2)$,
            $(italic("FV")(t_1) = V_1 space.quad italic("FV")(t_2) = V_2) / (italic("FV")(e ~> t_1 gt.tri t_2) = V_1 union V_2)$,
        ),
    ) <le_FV>

    #figure(
        caption: "代入規則の拡張",
        rules(
            $([x |-> t_0]t = t') / ([x |-> t_0]e angle.l t angle.r = e angle.l t' angle.r)$,
            $([x |-> t_0]t_1 = t'_1 space.quad [x |-> t_0]t_2 = t'_2) / ([x |-> t_0](e angle.l t_1 angle.r >>> t_2) = e angle.l t'_1 angle.r >>> t'_2)$,
            $([x |-> t_0]t_1 = t'_1 space.quad [x |-> t_0]t_2 = t'_2) / ([x |-> t_0](e ~> t_1 gt.tri t_2) = e ~> t'_1 gt.tri t'_2)$,
        ),
    ) <le_subst>

    #figure(
        caption: "1ステップ評価規則",
        rules(
            $(t_1 --> t'_1) / (t_1 space t_2 --> t'_1 space t_2)$,
            $(e angle.l t_1 angle.r >>> t_2) space t_3 --> e angle.l t_1 angle.r >>> lambda x_u. space (t_2 space x_u) space t_3$,
            $(forall t'_1. not (t_1 --> t'_1) space.quad t_2 --> t'_2 \ forall e. forall t_11. forall t_12. (e angle.l t_11 angle.r >>> t_12 != t_1)) / (t_1 space t_2 --> t_1 space t'_2)$,
            $(forall t'_1. not (t_1 --> t'_1) \ forall e_1. forall t_11. forall t_12. (e_1 angle.l t_11 angle.r >>> t_12 != t_1)) / (t_1 space (e_2 angle.l t_21 angle.r >>> t_22) --> e_2 angle.l t_21 angle.r >>> lambda x_u. space t_1 space (t_22 space x_u))$,
            $(forall t'_2. not (t_2 --> t'_2) space.quad forall e. forall t_21. forall t_22. (e angle.l t_21 angle.r >>> t_22 != t_2)) / ((lambda x. t_1) space t_2 --> [x |-> t_2]t_1)$,
            $(t --> t') / (e angle.l t angle.r --> e angle.l t' angle.r)$,
            $e_1 angle.l e_2 angle.l t_1 angle.r >>> t_2 angle.r --> e_2 angle.l t_1 angle.r >>> lambda x_u. space e_1 angle.l t_2 space x_u angle.r$,
            $(forall t'. not (t --> t') space.quad forall e_0. forall t_1. forall t_2. (e_0 angle.l t_1 angle.r >>> t_2 != t)) / (e angle.l t angle.r --> e angle.l t angle.r >>> lambda x_u. x_u)$,
            $(t_2 --> t'_2) / (e ~> t_1 gt.tri t_2 --> e ~> t_1 gt.tri t'_2)$,
            $e ~> t_1 gt.tri (e angle.l t_2 angle.r >>> t_3) \ --> (t_1 space t_2) space lambda x_u. (e ~> t_1 gt.tri (t_3 space x_u))$,
            $(e_1 != e_2) / (e_1 ~> t_1 gt.tri (e_2 angle.l t_2 angle.r >>> t_3) \ --> e_2 angle.l t_2 angle.r >>> lambda x_u. (e_1 ~> t_1 gt.tri (t_3 space x_u)))$,
            $(forall t'_2. not (t_2 --> t'_2) \ forall e_20. forall t_21. forall t_22. (e_20 angle.l t_21 angle.r >>> t_22 != t_2)) / (e ~> t_1 gt.tri t_2 --> t_2)$,
        ),
    ) <le_eval>

    = むすび

    #set heading(numbering: none)

    = 謝辞
    = 参考文献
]
