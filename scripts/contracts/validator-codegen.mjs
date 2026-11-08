/** Ajv 8.20 standalone code generation for the non-mutating contract profile.
 * The custom emitters follow Ajv's MIT-licensed keyword emitters
 * (https://github.com/ajv-validator/ajv), using independent validity temporaries
 * and explicit missing-property branches instead of assignments in conditions.
 * Copyright (c) 2015-2021 Evgeny Poberezkin; see AJV-LICENSE.
 * Keep the differential tests when upgrading Ajv: this uses its codegen API.
 */
import code from 'ajv/dist/vocabularies/code.js';
import util from 'ajv/dist/compile/util.js';
import {_, Name, not} from 'ajv/dist/compile/codegen/index.js';
import ts from 'typescript';
export function configureContractCodegen(ajv) {
  if (ajv.opts.allErrors || ajv.opts.$data || ajv.opts.useDefaults ||
      ajv.opts.removeAdditional) {
    throw new Error('Contract code generation requires the reviewed non-mutating, fail-fast profile');
  }
  // Definitions are instance-owned copies. Retain their rule order so
  // fail-fast error selection stays identical to unmodified Ajv.
  const anyOf=ajv.getKeyword('anyOf');
  const emitAnyOf=anyOf.code;
  anyOf.code = cxt=>{
    // The enclosing additionalProperties rule always marks every property as
    // evaluated, including when its schema is true. Union property maps would
    // be overwritten by that rule; child schemas still track their own maps.
    // Keep item tracking and branch/error evaluation unchanged.
    if(cxt.parentSchema.additionalProperties!==undefined)cxt.it.props=true;
    emitAnyOf(cxt);
  };
  const allOf=ajv.getKeyword('allOf');
  const discardedPropertySchemas=[];
  allOf.code = cxt=>{
    // As with anyOf, the enclosing additionalProperties rule supersedes
    // property maps from all branches. Nested schemas keep their own tracking.
    if(cxt.parentSchema.additionalProperties!==undefined)cxt.it.props=true;
    const {gen,schema,it}=cxt;
    const valid=gen.name('valid');
    schema.forEach((sch,i)=>{
      if(util.alwaysValidSchema(it,sch))return;
      // A branch still needs its annotations when it enforces unevaluated
      // properties itself. Otherwise the parent's fully evaluated sentinel
      // makes the branch's output map unnecessary. Scope this to this schema;
      // nested schemas and array-item annotations must remain untouched.
      discardedPropertySchemas.push(it.props===true && sch.unevaluatedProperties===undefined ? sch : undefined);
      let branch;
      try{branch=cxt.subschema({keyword:'allOf',schemaProp:i},valid);}
      finally{discardedPropertySchemas.pop();}
      cxt.ok(valid);
      cxt.mergeEvaluated(branch);
    });
  };
  const conditional=ajv.getKeyword('if');
  conditional.code = cxt=>{
    const {gen,parentSchema,it}=cxt;
    if(parentSchema.then===undefined && parentSchema.else===undefined)
      util.checkStrictMode(it,'"if" without "then" and "else" is ignored');
    const hasClause=keyword=>parentSchema[keyword]!==undefined && !util.alwaysValidSchema(it,parentSchema[keyword]);
    const hasThen=hasClause('then'),hasElse=hasClause('else');
    if(!hasThen && !hasElse)return;
    // Both branches assign valid when present; only a missing branch needs the
    // true default. Never overwrite the condition's temporary with clause results.
    const valid=gen.let('valid',hasThen && hasElse ? undefined : true);
    const condition=gen.name('_valid');
    const annotations=branch=>discardedPropertySchemas.at(-1)===parentSchema ? {...branch,props:undefined} : branch;
    const conditionCxt=cxt.subschema({keyword:'if',compositeRule:true,createErrors:false,allErrors:false},condition);
    cxt.mergeEvaluated(annotations(conditionCxt));
    cxt.reset();
    const clause=(keyword,ifClause)=>()=>{
      const clauseValid=gen.name('_valid');
      const clauseCxt=cxt.subschema({keyword},clauseValid);
      gen.assign(valid,clauseValid);
      cxt.mergeValidEvaluated(annotations(clauseCxt),valid);
      if(ifClause)gen.assign(ifClause,_`${keyword}`);
      else cxt.setParams({ifClause:keyword});
    };
    if(hasThen && hasElse){
      const ifClause=gen.let('ifClause');
      cxt.setParams({ifClause});
      gen.if(condition,clause('then',ifClause),clause('else',ifClause));
    }else if(hasThen)gen.if(condition,clause('then'));
    else gen.if(not(condition),clause('else'));
    cxt.pass(valid,()=>cxt.error(true));
  };
  const oneOf=ajv.getKeyword('oneOf');
  oneOf.code = cxt=>{
    const {gen,schema,parentSchema,it}=cxt;
    if(it.opts.discriminator && parentSchema.discriminator)return;
    const valid=gen.let('valid',false),passing=gen.let('passing',null);
    cxt.setParams({passing});
    gen.block(()=>schema.forEach((sch,i)=>{
      // Ajv reuses one function-scoped var for every branch. Independent
      // temporaries let optimization remove unused assignments without moving
      // the surviving declaration below an earlier branch's use.
      const schValid=gen.name('_valid');
      let schCxt;
      if(util.alwaysValidSchema(it,sch))gen.var(schValid,true);
      else schCxt=cxt.subschema({keyword:'oneOf',schemaProp:i,compositeRule:true},schValid);
      if(i>0)gen.if(_`${schValid} && ${valid}`)
        .assign(valid,false).assign(passing,_`[${passing}, ${i}]`).else();
      gen.if(schValid,()=>{
        gen.assign(valid,true).assign(passing,i);
        if(schCxt){
          // Assigning the fully evaluated sentinel is idempotent. Avoid a
          // self-read guard that keeps an otherwise unused property map alive.
          if(schCxt.props===true && it.props instanceof Name){
            gen.assign(it.props,true);
            cxt.mergeEvaluated({...schCxt,props:undefined},Name);
          }else cxt.mergeEvaluated(schCxt,Name);
        }
      });
    }));
    cxt.result(valid,()=>cxt.reset(),()=>cxt.error(true));
  };
  const properties=ajv.getKeyword('properties');
  properties.code = (cxt)=>{
    const {gen,schema,data,it}=cxt;
    const all=code.allSchemaProperties(schema);
    for(const prop of all)it.definedProperties.add(prop);
    if(it.opts.unevaluated && all.length && it.props!==true)
      it.props=util.mergeEvaluated.props(gen,util.toHash(all),it.props);
    for(const prop of all.filter(p=>!util.alwaysValidSchema(it,schema[p]))) {
      const valid=gen.name('valid');
      gen.if(code.propertyInData(gen,data,prop,it.opts.ownProperties));
      cxt.subschema({keyword:'properties',schemaProp:prop,dataProp:prop},valid);
      if(!it.allErrors)gen.else().var(valid,true);
      gen.endIf();cxt.it.definedProperties.add(prop);cxt.ok(valid);
    }
  };
  const required=ajv.getKeyword('required');
  required.code = (cxt)=>{
    if(cxt.it.opts.strictRequired) {
      for(const prop of cxt.schema) {
        if(cxt.parentSchema.properties?.[prop]===undefined && !cxt.it.definedProperties.has(prop)) {
          const schemaPath=cxt.it.schemaEnv.baseId+cxt.it.errSchemaPath;
          util.checkStrictMode(cxt.it,`required property "${prop}" is not defined at "${schemaPath}" (strictRequired)`,cxt.it.opts.strictRequired);
        }
      }
    }
    for(const prop of cxt.schema) {
      cxt.gen.if(code.noPropertyInData(cxt.gen,cxt.data,prop,cxt.it.opts.ownProperties));
      code.reportMissingProp(cxt,_`${prop}`);
      cxt.gen.else();
    }
  };
}
/** Internal error accumulators always use arrays. Public wrappers still expose
 * null on success. Failed nested validators supply error arrays; union rollback
 * truncates the accumulator to its previous length, including zero. This removes
 * nullable allocation guards without discarding or exposing error contents.
 */
export function normalizeErrorAccumulators(source) {
  const ast=ts.createSourceFile('validators.js',source,ts.ScriptTarget.ES2020,true,ts.ScriptKind.JS);
  const isErrors=n=>ts.isIdentifier(n)&&n.text==='vErrors';
  const isNull=n=>n?.kind===ts.SyntaxKind.NullKeyword;
  const result=ts.transform(ast,[ctx=>root=>{
    const visit=node=>{
      if(ts.isIfStatement(node)&&ts.isIdentifier(node.expression)&&node.elseStatement) {
        const unwrap=n=>ts.isBlock(n)&&n.statements.length===1?n.statements[0]:n;
        const yes=unwrap(node.thenStatement),no=unwrap(node.elseStatement);
        if(ts.isExpressionStatement(yes)&&ts.isBinaryExpression(yes.expression)&&ts.isExpressionStatement(no)&&ts.isBinaryExpression(no.expression)) {
          const a=yes.expression,b=no.expression;
          if(a.operatorToken.kind===ts.SyntaxKind.EqualsToken&&ts.isPropertyAccessExpression(a.left)&&isErrors(a.left.expression)&&a.left.name.text==='length'&&ts.isIdentifier(a.right)&&a.right.text===node.expression.text&&b.operatorToken.kind===ts.SyntaxKind.EqualsToken&&isErrors(b.left)&&isNull(b.right))return yes;
        }
      }
      if(ts.isVariableDeclaration(node)&&isErrors(node.name)&&isNull(node.initializer))
        return ctx.factory.updateVariableDeclaration(node,node.name,node.exclamationToken,node.type,ctx.factory.createArrayLiteralExpression());
      if(ts.isBinaryExpression(node)&&isErrors(node.left)&&isNull(node.right)) {
        if(node.operatorToken.kind===ts.SyntaxKind.EqualsToken)
          return ctx.factory.updateBinaryExpression(node,node.left,node.operatorToken,ctx.factory.createArrayLiteralExpression());
        if(node.operatorToken.kind===ts.SyntaxKind.EqualsEqualsEqualsToken)return ctx.factory.createFalse();
        if(node.operatorToken.kind===ts.SyntaxKind.ExclamationEqualsEqualsToken)return ctx.factory.createTrue();
      }
      return ts.visitEachChild(node,visit,ctx);
    };return ts.visitNode(root,visit);
  }]);
  try{return ts.createPrinter().printFile(result.transformed[0]);}finally{result.dispose();}
}

/** Ajv can repeat local error-count checks, and optimization can turn snapshots
 * into self-comparisons. Counters are finite integers, so loose comparisons are
 * safe here too. Property reads, calls and arbitrary expressions stay intact.
 */
export function normalizeRepeatedCounterChecks(source) {
  const ast = ts.createSourceFile('validators.js', source, ts.ScriptTarget.ES2020, true, ts.ScriptKind.JS);
  const atom = node => ts.isNumericLiteral(node) || (ts.isIdentifier(node) && /^(?:errors|_errs[0-9]+)$/.test(node.text));
  const comparison = node => ts.isBinaryExpression(node) &&
    [ts.SyntaxKind.EqualsEqualsEqualsToken, ts.SyntaxKind.ExclamationEqualsEqualsToken,
      ts.SyntaxKind.EqualsEqualsToken, ts.SyntaxKind.ExclamationEqualsToken].includes(node.operatorToken.kind) &&
    atom(node.left) && atom(node.right);
  const result = ts.transform(ast, [ctx => root => {
    const visit = node => {
      if(comparison(node) && node.left.getText(ast)===node.right.getText(ast))
        return [ts.SyntaxKind.EqualsEqualsEqualsToken,ts.SyntaxKind.EqualsEqualsToken].includes(node.operatorToken.kind)
          ? ctx.factory.createTrue() : ctx.factory.createFalse();
      if (ts.isBinaryExpression(node) &&
          [ts.SyntaxKind.AmpersandAmpersandToken, ts.SyntaxKind.BarBarToken].includes(node.operatorToken.kind) &&
          comparison(node.left) && comparison(node.right) && node.left.getText(ast) === node.right.getText(ast)) return visit(node.left);
      return ts.visitEachChild(node, visit, ctx);
    };
    return ts.visitNode(root, visit);
  }]);
  try { return ts.createPrinter().printFile(result.transformed[0]); }
  finally { result.dispose(); }
}

/** Remove the repeated object guard emitted by not after a fail-fast type guard.
 * Only cross declarations of counter snapshots, with no calls or assignments.
 * This deliberately does not perform general control-flow/type inference.
 */
export function normalizeKnownObjectChecks(source) {
  const ast=ts.createSourceFile('validators.js',source,ts.ScriptTarget.ES2020,true,ts.ScriptKind.JS);
  const compact=node=>node.getText(ast).replace(/\s+/g,'');
  const result=ts.transform(ast,[ctx=>root=>{
    const visit=node=>{
      if(!ts.isBlock(node))return ts.visitEachChild(node,visit,ctx);
      let known;
      const statements=node.statements.map(statement=>{
        let updated=statement;
        if(known && ts.isIfStatement(statement) && !statement.elseStatement &&
            /^errors===_errs\d+$/.test(compact(statement.expression))) {
          const inner=statement.thenStatement;
          if(ts.isIfStatement(inner) && compact(inner.expression)===`${known}&&"object"==typeof${known}&&!Array.isArray(${known})`)
            updated=ctx.factory.updateIfStatement(statement,statement.expression,inner.thenStatement,undefined);
        }
        if(ts.isIfStatement(statement) && !statement.elseStatement && ts.isReturnStatement(statement.thenStatement)) {
          const match=compact(statement.expression).match(/^!(data\d*)\|\|"object"!=typeof\1\|\|Array\.isArray\(\1\)$/);
          known=match?.[1];
        }else if(!(ts.isVariableStatement(statement) && statement.declarationList.declarations.every(declaration=>
          ts.isIdentifier(declaration.name) && /^_errs\d+$/.test(declaration.name.text) &&
          declaration.initializer && ts.isIdentifier(declaration.initializer) && declaration.initializer.text==='errors')))known=undefined;
        return ts.visitEachChild(updated,visit,ctx);
      });
      return ctx.factory.updateBlock(node,statements);
    };
    return ts.visitNode(root,visit);
  }]);
  try{return ts.createPrinter().printFile(result.transformed[0]);}finally{result.dispose();}
}
